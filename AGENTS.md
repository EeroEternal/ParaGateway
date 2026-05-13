# AI Agent 协作规则

## 项目规范

### ParaGateway 与 UniGateway 边界

- **ParaGateway 是企业 AI 资源控制平面**，负责产品层配置、组织/项目/API Key、Provider Account、Endpoint、Virtual Model、Model Pool、数据库同步、路由策略、鉴权、配额、Token 统计、健康检查和 UI 管理逻辑。
- **UniGateway 是协议与执行引擎**，负责协议层语义转换、provider driver、上游请求渲染、下游响应归一化、流式协议处理、SSE chunk 解析/重写和具体 provider 行为。
- **业务对象不得下沉到 UniGateway**：Org、Project、API Key、Provider Account、Model Pool、Virtual Model、Project Grant 等属于 ParaGateway；UniGateway 不应依赖或理解这些产品层对象。
- **协议细节不得上浮到 ParaGateway**：OpenAI/Anthropic 协议字段转换、provider-specific 请求体/请求头渲染、reasoning/thinking 字段解析、工具调用差异、流式响应差异等应优先在 UniGateway 中实现。
- ParaGateway 可以维护 Endpoint 候选集、健康状态、优先级、权重、运行时指标和路由反馈分数，并将中立 metadata 或 `unigateway.*` hints 传给 UniGateway；但不得在 API handler 中把这些 metadata 翻译成特定 provider 的 body 参数。
- ParaGateway 的路由职责是 **策略计算**：根据 Virtual Model 解析 Model Pool，过滤可用 Endpoint，计算 Priority/Latency-Based/Least-Connections 等分数，并生成 UniGateway 可执行的候选 Endpoint 顺序。
- UniGateway 的路由职责是 **执行机制**：根据 ParaGateway 提供的候选 Endpoint、反馈分数和中立 metadata 完成协议渲染、请求执行、fallback、响应归一化和报告输出。
- 如果 UniGateway 当前 fallback、tie-breaking、ordered endpoints 或 driver 行为不足，应优先在 UniGateway 中增强机制，而不是在 ParaGateway API handler 中硬编码 provider 或协议逻辑。
- API Key 应绑定 Project，Project 授权 Virtual Model，Virtual Model 映射 Model Pool，Model Pool 聚合 Endpoint；避免设计成 API Key 直接绑定 Endpoint 或直接承载 provider-specific 行为。
- 修复前应先判断问题属于产品配置/路由数据，还是协议适配/driver 渲染；边界不清时先说明归属判断，不要把 UniGateway 责任下沉到 ParaGateway。

### 文件组织

- **严禁在根目录放置临时或测试脚本**。
- 所有测试脚本应根据所属模块放置在对应目录的 `tests/` 或 `scripts/` 下（例如 `src/tests/`）。
- 根目录应保持整洁，仅包含项目配置文件及必要的说明文档。

### 文档命名与归档

- `docs/` 下的文档文件名应保持简短清晰，避免使用过长的复合名称。
- 当文档需要表达模块、阶段、方案、RFC 等附加区分时，优先使用子目录分类，而不是继续拉长文件名。
- 新增文档时，优先放入对应主题目录，例如 `docs/unigateway/`、`docs/ui/`、`docs/design/`。
- 文档重构后应保持 `docs/` 顶层目录可快速浏览，必要时更新索引文档。

### 前端 UI

#### 下拉选择

产品界面中的选项列表（状态、筛选条件、枚举字段等）**不得使用原生 HTML `<select>`**，应使用项目内封装组件 `web/src/components/Select.tsx`（或经评审的同等可访问自定义下拉），以保持样式一致并避免浏览器默认控件在弹窗、主题与交互上与整体设计脱节。

#### 多语言规范

- 所有新增的 UI 文本（包括标签、提示语、CTA 等）**必须同步更新至所有现存的本地化文件** (`en.json`, `zh.json`, `ja.json`, `ko.json`)。
- 禁止在前端组件中直接硬编码中文或英文文案。
- 如果不确定翻译，请优先提供准确的英文，并明确标注待补充语种，避免仅维护单一语言。

## 部署纪律


### 代码提交后必须监控部署状态

任何代码推送到 main 分支后，必须立即监控 GitHub Actions 部署状态，直到确认成功或失败。

**执行步骤**：
1. 推送代码后立即获取最新 workflow run ID
2. 使用 `gh run watch <run-id>` 实时监控
3. 确认 deployment job 成功完成

**命令示例**：
```bash
# 推送代码后获取 run ID
gh run list -L 1 --json databaseId

# 监控部署状态
gh run watch <databaseId>
```

**失败处理**：
- 若部署失败，立即查看日志定位问题
- 修复后重新提交并再次监控
- 不要将监控任务留给用户
