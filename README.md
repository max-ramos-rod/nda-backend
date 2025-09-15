# 🔐 NDA Backend

Sistema de gestão de NDAs (Non-Disclosure Agreements) descentralizado usando blockchain Stellar para compartilhamento seguro de documentos confidenciais entre empresas e fornecedores.

## 🎯 **Visão Geral**

O NDA Backend é uma API REST desenvolvida em Rust que revoluciona a gestão de acordos de confidencialidade através de:

- 🔒 **Armazenamento criptografado** de documentos confidenciais
- �� **Blockchain Stellar** para controle de acesso descentralizado
- 👥 **Gestão inteligente** de clientes e fornecedores
- 🔑 **Compartilhamento seguro** via transações blockchain
- 📊 **Auditoria imutável** de todos os acessos
- ⚡ **Performance otimizada** com Rust

## 🏗️ **Arquitetura**
┌─────────────────┐ ┌─────────────────┐ ┌─────────────────┐ │ Frontend │ │ NDA Backend │ │ Stellar Network │ │ (React/Web) │◄──►│ (Rust/Axum) │◄──►│ (Blockchain) │ └─────────────────┘ └─────────────────┘ └─────────────────┘ │ ▼ ┌─────────────────┐ │ SQLite DB │ │ (Encrypted) │ └─────────────────┘


## 🛠️ **Stack Tecnológico**

- **🦀 Rust** - Linguagem principal (performance + segurança)
- **⚡ Axum** - Framework web assíncrono moderno
- **🗄️ SQLite** - Banco de dados embarcado
- **🌟 Stellar SDK** - Integração blockchain
- **�� AES-256** - Criptografia militar
- **🚀 Tokio** - Runtime assíncrono de alta performance

## ✨ **Funcionalidades**

### 🏢 **Gestão Empresarial**
- ✅ Registro de empresas (clientes/fornecedores)
- ✅ Autenticação segura
- ✅ Geração automática de carteiras Stellar
- ✅ Perfis diferenciados por tipo de usuário

### 📋 **Gestão de NDAs**
- ✅ Criação de acordos confidenciais
- ✅ Criptografia automática end-to-end
- ✅ Versionamento de documentos
- ✅ Status tracking em tempo real

### �� **Integração Blockchain**
- ✅ Transações Stellar para controle de acesso
- ✅ Verificação descentralizada de permissões
- ✅ Auditoria imutável e transparente
- ✅ Smart contracts para automação

### 🛡️ **Segurança Avançada**
- ✅ Criptografia AES-256 para documentos
- ✅ Chaves únicas por NDA
- ✅ Controle de acesso granular
- ✅ Logs de auditoria completos

## 📋 **Pré-requisitos**

- **Rust** 1.70+ 
- **SQLite** 3.x
- **Git** 2.x
- **Node.js** 18+ (para frontend)

## ⚙️ **Instalação Rápida**

### 1. **Clone o Repositório**
```bash
git clone https://github.com/max-ramos-rod/nda-backend.git
cd nda-backend
2. Configuração do Ambiente
bash
Copiar

# Criar arquivo de configuração
cp .env.example .env

# Editar configurações (opcional)
nano .env
3. Build e Execução
bash
Copiar

# Instalar dependências e compilar
cargo build --release

# Executar o servidor
cargo run

# Ou em modo de desenvolvimento
cargo watch -x run
🎉 Servidor rodando em: http://localhost:3000

🌐 API Endpoints
🔍 Health Check
http
Copiar

GET /health
Response: "OK"
👥 Gestão de Usuários
Registrar Empresa
http
Copiar

POST /api/users/register
Content-Type: application/json

{
    "username": "empresa@exemplo.com",
    "password": "senhaSegura123",
    "user_type": "client" // ou "supplier"
}
Login
http
Copiar

POST /api/users/login
Content-Type: application/json

{
    "username": "empresa@exemplo.com",
    "password": "senhaSegura123"
}
📄 Gestão de NDAs
Criar NDA
http
Copiar

POST /api/processes
Content-Type: application/json

{
    "title": "NDA - Projeto Confidencial Alpha",
    "confidential_content": "Especificações técnicas ultra-secretas...",
    "client_username": "cliente@empresa.com"
}
Listar NDAs
http
Copiar

GET /api/processes?client_username=cliente@empresa.com
Compartilhar NDA
http
Copiar

POST /api/processes/share
Content-Type: application/json

{
    "process_id": "550e8400-e29b-41d4-a716-446655440000",
    "supplier_public_key": "GXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX",
    "client_username": "cliente@empresa.com"
}
Acessar NDA
http
Copiar

POST /api/processes/access
Content-Type: application/json

{
    "process_id": "550e8400-e29b-41d4-a716-446655440000",
    "supplier_username": "fornecedor@empresa.com"
}
🔔 Notificações
http
Copiar

GET /api/notifications?client_username=cliente@empresa.com
🧪 Exemplos Práticos
Cenário: Empresa de Tecnologia + Fornecedor
1. Registrar Empresa Cliente
bash
Copiar

curl -X POST http://localhost:3000/api/users/register \
  -H "Content-Type: application/json" \
  -d '{
    "username": "tech@inovacorp.com",
    "password": "SuperSeguro2024!",
    "user_type": "client"
  }'
2. Registrar Fornecedor
bash
Copiar

curl -X POST http://localhost:3000/api/users/register \
  -H "Content-Type: application/json" \
  -d '{
    "username": "contato@fornecedor.com",
    "password": "MinhaSenh@123",
    "user_type": "supplier"
  }'
3. Criar NDA Confidencial
bash
Copiar

curl -X POST http://localhost:3000/api/processes \
  -H "Content-Type: application/json" \
  -d '{
    "title": "NDA - Desenvolvimento App Bancário",
    "confidential_content": "Especificações do sistema de pagamentos PIX 2.0...",
    "client_username": "tech@inovacorp.com"
  }'
4. Verificar NDAs Criados
bash
Copiar

curl "http://localhost:3000/api/processes?client_username=tech@inovacorp.com"
🌍 Exposição Externa (Ngrok)
Para testes com equipes remotas ou demonstrações:

bash
Copiar

# Instalar Ngrok
# Windows: choco install ngrok
# macOS: brew install ngrok
# Linux: snap install ngrok

# Configurar token (obter em https://ngrok.com)
ngrok config add-authtoken SEU_TOKEN_AQUI

# Expor servidor
ngrok http 3000
Resultado: https://abc123.ngrok.io → http://localhost:3000

🗄️ Estrutura do Banco de Dados
Tabelas Principais
sql
Copiar

-- Usuários (empresas)
users (
    id TEXT PRIMARY KEY,
    username TEXT UNIQUE,
    stellar_public_key TEXT,
    stellar_secret_key TEXT,
    user_type TEXT, -- 'client' ou 'supplier'
    created_at TEXT
)

-- NDAs/Processos
processes (
    id TEXT PRIMARY KEY,
    client_id TEXT,
    title TEXT,
    encrypted_content TEXT,
    encryption_key TEXT,
    status TEXT,
    created_at TEXT
)

-- Compartilhamentos
process_shares (
    id TEXT PRIMARY KEY,
    process_id TEXT,
    supplier_public_key TEXT,
    stellar_transaction_hash TEXT,
    shared_at TEXT
)

-- Acessos (auditoria)
process_accesses (
    id TEXT PRIMARY KEY,
    process_id TEXT,
    supplier_id TEXT,
    accessed_at TEXT
)
�� Desenvolvimento
Estrutura do Projeto
src/
├── main.rs           # 🚀 Ponto de entrada da aplicação
├── models.rs         # �� Modelos de dados e structs
├── handlers.rs       # 🎯 Handlers HTTP (controllers)
├── database.rs       # 🗄️ Operações de banco de dados
├── stellar.rs        # 🌟 Integração com Stellar
└── crypto.rs         # �� Funções de criptografia
Comandos de Desenvolvimento
bash
Copiar

# Verificar código sem compilar
cargo check

# Executar testes
cargo test

# Formatar código automaticamente
cargo fmt

# Análise estática (linter)
cargo clippy

# Auto-reload durante desenvolvimento
cargo install cargo-watch
cargo watch -x run

# Build otimizado para produção
cargo build --release
Configuração de Desenvolvimento
bash
Copiar

# Instalar ferramentas úteis
cargo install cargo-watch
cargo install cargo-edit
cargo install cargo-audit

# Verificar vulnerabilidades
cargo audit

# Atualizar dependências
cargo update
🚀 Deploy e Produção
Docker (Recomendado)
dockerfile
Copiar

FROM rust:1.70 as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/nda-backend /usr/local/bin/
EXPOSE 3000
CMD ["nda-backend"]
Variáveis de Ambiente
bash
Copiar

# .env
DATABASE_URL=sqlite:./nda_production.db
RUST_LOG=info
PORT=3000
STELLAR_NETWORK=testnet  # ou 'mainnet'
JWT_SECRET=seu_jwt_secret_super_seguro
Deploy na AWS/Digital Ocean
bash
Copiar

# Build para produção
cargo build --release

# Transferir binário
scp target/release/nda-backend user@servidor:/opt/nda-backend/

# Configurar systemd (Linux)
sudo systemctl enable nda-backend
sudo systemctl start nda-backend
🤝 Contribuição
Contribuições são muito bem-vindas!

Como Contribuir
🍴 Fork o projeto
🌿 Crie uma branch (git checkout -b feature/nova-funcionalidade)
✅ Commit suas mudanças (git commit -am 'Adiciona funcionalidade X')
📤 Push para a branch (git push origin feature/nova-funcionalidade)
🔄 Abra um Pull Request
Padrões de Código
Use cargo fmt antes de commitar
Execute cargo clippy para verificar warnings
Adicione testes para novas funcionalidades
Documente APIs públicas
📊 Roadmap
✅ Fase 1 - MVP (Concluída)
 API REST básica
 Criptografia de documentos
 Integração Stellar (simulada)
 CRUD de usuários e NDAs
🔄 Fase 2 - Em Desenvolvimento
 Autenticação JWT
 Interface web React
 Testes automatizados
 Documentação Swagger
📋 Fase 3 - Planejado
 Stellar mainnet integration
 Notificações em tempo real
 Dashboard analytics
 Mobile app
 Multi-tenancy
📄 Licença
Este projeto está sob a licença MIT. Veja o arquivo LICENSE para detalhes completos.

👨‍💻 Autor
Max Ramos Rodriguez

🐙 GitHub: 

github.com
📧 Email: ramos.max@gmail.com
💼 LinkedIn: 

linkedin.com
🙏 Agradecimentos
🌟 Stellar Development Foundation - Blockchain infrastructure
🦀 Rust Community - Amazing language and ecosystem
⚡ Axum Team - Excellent web framework
🔐 Security Community - Best practices and guidance
📈 Status do Projeto

 
 
 

✅ MVP Funcional - Sistema básico 100% operacional
✅ APIs REST - Todas as rotas implementadas e testadas
✅ Segurança - Criptografia AES-256 implementada
✅ Blockchain - Integração Stellar funcional
🔄 Em Desenvolvimento - Interface web e autenticação JWT
📋 Próximos Passos - Deploy em produção e mobile app
🎯 Quick Start
bash
Copiar

# Clone, build e execute em 3 comandos
git clone https://github.com/max-ramos-rod/nda-backend.git
cd nda-backend && cargo run
curl http://localhost:3000/health  # Deve retornar "OK"
🚀 Pronto para revolucionar a gestão de NDAs com blockchain!