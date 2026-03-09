# 🗺️ Mapa de Dependências: temp/composio/core

## 📊 Diagrama de Dependências

```
┌─────────────────────────────────────────────────────────────┐
│                      types.py                               │
│  ToolkitVersion, ToolkitVersions, ToolkitVersionParam       │
└────────────────────┬────────────────────────────────────────┘
                     │
                     │ Usado por ↓
                     │
        ┌────────────┼────────────┬────────────┐
        │            │            │            │
        ▼            ▼            ▼            ▼
┌──────────┐  ┌──────────┐  ┌──────────┐  ┌──────────┐
│ tools.py │  │triggers.py│  │ sdk.py  │  │ utils/   │
│          │  │          │  │          │  │toolkit_  │
│          │  │          │  │          │  │version.py│
└──────────┘  └──────────┘  └──────────┘  └──────────┘
```

---

## 🔗 Cadeia de Dependências Detalhada

### 1. **base.py** (Fundação)
```
base.py
├─ Depende de:
│  ├─ HttpClient (composio.client)
│  ├─ WithLogger (composio.utils.logging)
│  └─ _telemetry.py (create_event, push_event)
│
└─ Usado por: TODOS os outros models
   ├─ auth_configs.py
   ├─ connected_accounts.py
   ├─ tools.py
   ├─ toolkits.py
   ├─ tool_router.py
   ├─ triggers.py
   ├─ mcp.py
   └─ internal.py
```

### 2. **types.py** (Tipos de Versionamento)
```
types.py
├─ Depende de:
│  ├─ typing
│  └─ typing_extensions
│
└─ Usado por:
   ├─ tools.py (ToolkitVersionParam)
   ├─ triggers.py (ToolkitVersionParam)
   ├─ sdk.py (ToolkitVersionParam)
   └─ utils/toolkit_version.py (todos os tipos)
```

### 3. **_files.py** (Gerenciamento de Arquivos)
```
_files.py
├─ Depende de:
│  ├─ HttpClient
│  ├─ WithLogger
│  ├─ requests
│  └─ hashlib (MD5)
│
└─ Usado por:
   └─ tools.py (FileHelper)
```

### 4. **_modifiers.py** (Modificadores)
```
_modifiers.py
├─ Depende de:
│  ├─ typing (Protocols)
│  └─ functools
│
└─ Usado por:
   ├─ tools.py (Modifiers, before_execute, after_execute, schema_modifier)
   └─ tool_router.py (Modifiers)
```

### 5. **_telemetry.py** (Telemetria)
```
_telemetry.py
├─ Depende de:
│  ├─ httpx
│  ├─ queue
│  ├─ threading
│  └─ atexit
│
└─ Usado por:
   └─ base.py (create_event, push_event)
```

### 6. **connected_accounts.py**
```
connected_accounts.py
├─ Depende de:
│  ├─ base.py (Resource)
│  ├─ HttpClient
│  └─ exceptions
│
└─ Usado por:
   ├─ toolkits.py (ConnectedAccounts)
   └─ tool_router.py (ConnectionRequest)
```

### 7. **auth_configs.py**
```
auth_configs.py
├─ Depende de:
│  ├─ base.py (Resource)
│  └─ HttpClient
│
└─ Usado por:
   └─ toolkits.py (indiretamente via client)
```

### 8. **custom_tools.py**
```
custom_tools.py
├─ Depende de:
│  ├─ HttpClient
│  ├─ pydantic (BaseModel)
│  └─ inspect
│
└─ Usado por:
   └─ tools.py (CustomTool, CustomTools)
```

### 9. **webhook_events.py**
```
webhook_events.py
├─ Depende de:
│  ├─ typing
│  └─ enum
│
└─ Usado por:
   ├─ triggers.py (WebhookVersion, WebhookPayload)
   └─ models/__init__.py (exports)
```

### 10. **internal.py**
```
internal.py
├─ Depende de:
│  ├─ base.py (Resource)
│  └─ composio_client (BaseModel)
│
└─ Usado por:
   └─ triggers.py (get_sdk_realtime_credentials)
```

### 11. **toolkits.py**
```
toolkits.py
├─ Depende de:
│  ├─ base.py (Resource)
│  ├─ connected_accounts.py (ConnectedAccounts)
│  └─ HttpClient
│
└─ Usado por:
   └─ tool_router.py (authorize, get_auth_fields)
```

### 12. **tools.py** (Hub Central)
```
tools.py
├─ Depende de:
│  ├─ base.py (Resource)
│  ├─ _files.py (FileHelper)
│  ├─ _modifiers.py (Modifiers, decorators)
│  ├─ custom_tools.py (CustomTools)
│  ├─ types.py (ToolkitVersionParam)
│  ├─ provider/ (BaseProvider, AgenticProvider, NonAgenticProvider)
│  └─ utils/toolkit_version.py (get_toolkit_version)
│
└─ Usado por:
   └─ tool_router.py (Tools)
```

### 13. **triggers.py**
```
triggers.py
├─ Depende de:
│  ├─ base.py (Resource)
│  ├─ internal.py (Internal)
│  ├─ types.py (ToolkitVersionParam)
│  ├─ webhook_events.py (WebhookVersion)
│  ├─ pysher (Pusher)
│  └─ utils/toolkit_version.py (get_toolkit_version)
│
└─ Usado por:
   └─ sdk.py (Triggers)
```

### 14. **mcp.py**
```
mcp.py
├─ Depende de:
│  ├─ base.py (Resource)
│  └─ HttpClient
│
└─ Usado por:
   └─ tool_router.py (MCP info)
```

### 15. **tool_router.py** (Orquestrador)
```
tool_router.py
├─ Depende de:
│  ├─ base.py (Resource)
│  ├─ connected_accounts.py (ConnectionRequest)
│  ├─ tools.py (Tools)
│  ├─ _modifiers.py (Modifiers)
│  ├─ provider/ (BaseProvider)
│  └─ HttpClient
│
└─ Usado por:
   └─ sdk.py (ToolRouter, create session)
```

---

## 🎯 Ordem de Dependência (Bottom-Up)

### Nível 1: Fundação (Sem dependências internas)
```
1. types.py                    ← Tipos puros
2. _telemetry.py              ← Sistema independente
3. webhook_events.py          ← Tipos puros
```

### Nível 2: Base (Depende apenas de Nível 1)
```
4. base.py                    ← Usa _telemetry.py
5. internal.py                ← Usa base.py
```

### Nível 3: Utilitários (Depende de Nível 1-2)
```
6. _files.py                  ← Usa base.py
7. _modifiers.py              ← Tipos puros
8. custom_tools.py            ← Independente
```

### Nível 4: Recursos Básicos (Depende de Nível 1-3)
```
9. auth_configs.py            ← Usa base.py
10. connected_accounts.py     ← Usa base.py
```

### Nível 5: Recursos Intermediários (Depende de Nível 1-4)
```
11. toolkits.py               ← Usa base.py, connected_accounts.py
12. mcp.py                    ← Usa base.py
```

### Nível 6: Recursos Avançados (Depende de Nível 1-5)
```
13. tools.py                  ← Usa base.py, _files.py, _modifiers.py, 
                                 custom_tools.py, types.py
14. triggers.py               ← Usa base.py, internal.py, types.py, 
                                 webhook_events.py
```

### Nível 7: Orquestração (Depende de tudo)
```
15. tool_router.py            ← Usa base.py, connected_accounts.py, 
                                 tools.py, _modifiers.py
```

---

## 🔄 Fluxo de Dados Típico

### Cenário 1: Criar Sessão e Executar Ferramenta
```
1. SDK.create(user_id)
   └─> tool_router.py: ToolRouter.create()
       └─> HttpClient: POST /tool_router/session
           └─> Retorna: ToolRouterSession

2. session.tools()
   └─> tool_router.py: ToolRouterSession.tools()
       └─> tools.py: Tools.get()
           └─> HttpClient: GET /tool_router/session/{id}/tools
               └─> Retorna: List[Tool] (formatado pelo provider)

3. session.execute_tool(slug, args)
   └─> tool_router.py: ToolRouterSession.execute_tool()
       ├─> _modifiers.py: before_execute (se configurado)
       ├─> _files.py: upload_file (se necessário)
       ├─> tools.py: Tools.execute()
       │   └─> HttpClient: POST /tool_router/session/{id}/execute
       ├─> _files.py: download_file (se necessário)
       └─> _modifiers.py: after_execute (se configurado)
           └─> Retorna: ToolExecutionResponse
```

### Cenário 2: Autenticar Toolkit
```
1. session.authorize(toolkit)
   └─> tool_router.py: ToolRouterSession.authorize()
       └─> toolkits.py: Toolkits.authorize()
           └─> connected_accounts.py: ConnectedAccounts.create()
               └─> HttpClient: POST /connected_accounts
                   └─> Retorna: ConnectionRequest

2. connection_request.wait_for_connection()
   └─> connected_accounts.py: ConnectionRequest.wait_for_connection()
       └─> Loop: HttpClient: GET /connected_accounts/{id}
           └─> Retorna: ConnectedAccountRetrieveResponse (quando ACTIVE)
```

### Cenário 3: Criar e Escutar Trigger
```
1. triggers.create(slug, user_id, config)
   └─> triggers.py: Triggers.create()
       ├─> utils/toolkit_version.py: get_toolkit_version()
       └─> HttpClient: POST /trigger_instances/{slug}/upsert
           └─> Retorna: TriggerInstanceUpsertResponse

2. triggers.subscribe(handler)
   └─> triggers.py: Triggers.subscribe()
       ├─> internal.py: Internal.get_sdk_realtime_credentials()
       ├─> pysher: Pusher.connect()
       └─> Loop: Escuta eventos
           └─> Chama: handler(trigger_data)
```

---

## 📦 Agrupamento por Funcionalidade

### Grupo 1: Autenticação
```
├─ auth_configs.py          (Configurações de auth)
├─ connected_accounts.py    (Contas conectadas)
└─ toolkits.py             (Autorização de toolkits)
```

### Grupo 2: Execução de Ferramentas
```
├─ tools.py                (Gerenciamento e execução)
├─ custom_tools.py         (Ferramentas customizadas)
├─ _files.py              (Upload/download)
└─ _modifiers.py          (Modificadores)
```

### Grupo 3: Sessões e Roteamento
```
├─ tool_router.py         (Sessões do Tool Router)
└─ mcp.py                 (Model Control Protocol)
```

### Grupo 4: Eventos e Triggers
```
├─ triggers.py            (Gerenciamento de triggers)
├─ webhook_events.py      (Tipos de eventos)
└─ internal.py           (APIs internas)
```

### Grupo 5: Infraestrutura
```
├─ base.py               (Classe base Resource)
├─ types.py              (Tipos de versionamento)
└─ _telemetry.py        (Telemetria)
```

---

## 🎓 Explicação Didática

### Como os arquivos se relacionam?

Imagine uma **fábrica de ferramentas**:

1. **base.py** = Fundação da fábrica
   - Todos os departamentos (models) são construídos sobre ela

2. **types.py** = Manual de versões
   - Define quais versões de ferramentas existem

3. **auth_configs.py** = Departamento de Credenciais
   - Guarda as chaves e senhas

4. **connected_accounts.py** = Portaria
   - Verifica quem pode entrar (OAuth, API Keys)

5. **toolkits.py** = Catálogo de Ferramentas
   - Lista todas as ferramentas disponíveis

6. **tools.py** = Linha de Produção
   - Executa as ferramentas
   - Usa _files.py para arquivos
   - Usa _modifiers.py para customizar

7. **tool_router.py** = Gerente de Projetos
   - Coordena tudo
   - Cria sessões
   - Distribui trabalho

8. **triggers.py** = Sistema de Alarmes
   - Escuta eventos
   - Notifica quando algo acontece

9. **_files.py** = Almoxarifado
   - Guarda e busca arquivos

10. **_modifiers.py** = Oficina de Customização
    - Modifica ferramentas antes/depois de usar

11. **_telemetry.py** = Sistema de Monitoramento
    - Registra tudo que acontece

---

## 🚀 Ordem de Implementação Recomendada

### Fase 1: Fundação (1-2 dias)
```
1. types.py              → src/models/versioning.rs
2. webhook_events.py     → src/models/webhook_events.rs
```

### Fase 2: Utilitários (2-3 dias)
```
3. _files.py            → src/utils/files.rs
4. _modifiers.py        → src/models/modifiers.rs
```

### Fase 3: Recursos Avançados (2-3 dias)
```
5. custom_tools.py      → src/models/custom_tools.rs
6. triggers.py (completo) → src/models/triggers.rs
```

### Fase 4: Infraestrutura (1-2 dias)
```
7. _telemetry.py        → src/utils/telemetry.rs
8. internal.py          → src/models/internal.rs
```

---

Quer que eu comece pela **Fase 1** implementando `types.py` (versionamento)? 🎯
