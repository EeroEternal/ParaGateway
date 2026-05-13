# UniGateway 优化原语提案 (已对齐库边界)

本档根据 ParaGateway 的企业级需求，提炼出适用于 UniGateway 库层次的通用优化原语。核心原则是：**UniGateway 提供底层的限流、信号、指标与透传原语；ParaGateway 宿主层负责具体的业务策略（如 LeastConnections 算法、企业级评分公式等）。**

---

## 1. 端点物理并发硬限 (Endpoint Static Concurrency) - P1

### 描述
在企业级场景中，上游端点（如 Azure 部署、私有化模型实例）通常有严格的物理并发配额。目前 UniGateway 仅支持 AIMD 自适应控制，缺乏静态硬限。

### 优化提案
- **数据结构**：在 `unigateway_core::pool::Endpoint` 中增加 `max_concurrency: Option<usize>` 字段。
- **引擎逻辑**：不引入额外的检查分叉，而是将此硬限融入 AIMD 的 `effective_limit` 计算。
  - 公式：`effective_limit = min(adaptive_current_limit, endpoint_max_concurrency.unwrap_or(global_max))`
- **收益**：避免在高负载或网络抖动时，AIMD 升速超过服务商的物理阈值，确保企业级生产环境的稳定性。

---

## 2. 严格按分值排序策略 (ScoreOrdered Strategy) - P2

### 描述
ParaGateway 需要实现基于延迟、价格或人工权重的精确调度。现有的 `Fallback` 策略在语义上偏向“备选尝试”，且代码中可能存在对 Feedback 结果的微小扰动。

### 优化提案
- **数据结构**：在 `LoadBalancingStrategy` 枚举中新增 `ScoreOrdered` 成员。
- **调度逻辑**：
  - 该策略指示引擎**严格遵循** Feedback Provider 提供的 `score` 降序排列。
  - 明确禁用任何类似于 `Random` 的打散或 `RoundRobin` 的旋转行为。
  - 对于分值相同的端点，保持确定的平局裁决（如 `endpoint_id` 字典序），以便于审计和调试。
- **收益**：为 ParaGateway 实现各种基于“分值”的调度策略（如最少连接、最低价格）提供确定的底层机制。

---

## 3. 并发观测指标增强 (Endpoint Metrics Primitives) - P1

### 描述
为了在 ParaGateway 侧实现 `Least Connections`（最少连接）调度算法，宿主层需要知道 UniGateway 引擎内部真实的端点负载情况。

### 优化提案
- **Hook 增强**：在 `unigateway_core::hooks::AttemptStartedEvent` 中增加 `endpoint_active_attempts_at_start: usize` 字段。
- **实现方案**：在 Attempt 启动时，对 `AdaptiveConcurrency` 维护的 `active_connections` 原子计数器进行快照并随事件抛出。
- **收益**：提供中立的观测指标。ParaGateway 只需订阅 Hook，即可根据此“库内真实负载”计算评分，无需在宿主层重复维护计数逻辑。

---

## 4. 链路指标一致性 (Latency/TTFT Consistency) - P1

### 描述
企业级监控需要精确的 TTFT (Time To First Token) 和总延迟数据。

### 优化提案
- **规范化**：确保所有 Driver（OpenAI/Anthropic）在流式请求中一致填充 `StreamReport.ttft_ms`。
- **观测边界**：明确 `ttft_ms` 仅在流式路径（Streaming Path）存在，非流式路径仅提供 `latency_ms`。

---

## 5. 边界确认与审计记录

### Metadata 透传 (已确认)
- **结论**：经审计，UniGateway 的 Metadata 合并路径（Snapshot -> Endpoint -> Request）已覆盖所有生命周期 Hook 和 Report。
- **后续**：ParaGateway 应在构造 `ProxyChatRequest` 时正确注入 `org_id` 和 `project_id`，无需对库代码进行改动。

### 驱动拦截 (暂不实施)
- **结论**：企业级 Header 注入（如 Trace ID）可通过现有的 `on_request` 钩子修改 `metadata`，再由驱动层映射为 HTTP Header 解决。
- **后续**：暂不需要在 `GatewayHooks` 中暴露 Raw HTTP 修改能力，保持协议层与传输层的解耦。

---

## 实施建议顺序
1. **P1**：端点静态并发上限 & 并发观测指标增强 (这是实现调度优化的物理基础)。
2. **P1**：指标一致性规范化。
3. **P2**：`ScoreOrdered` 策略 (这是实现高级权重的机制保证)。
