# TechStore — E-commerce Full Stack

Este projeto é um sistema completo de e-commerce e carrinho de compras construído com tecnologias modernas. O principal objetivo é demonstrar uma arquitetura de software sólida e de alta performance, utilizando Rust no ecossistema de backend e React no frontend.

---

## Arquitetura e Organização

A aplicação foi estruturada seguindo o padrão de arquitetura em camadas, garantindo a separação de responsabilidades e facilitando a manutenção e testes individuais de cada módulo.

```
[Frontend (React)] ─► [Controllers (Rocket)] ─► [Services (Negócio)] ─► [Repositories (Diesel)] ─► [SQLite]
```

*   **Controllers (Camada de Entrega):** Responsável por lidar com as rotas HTTP, mapear os parâmetros das requisições e validar os dados de entrada (DTOs).
*   **Services (Camada de Negócio):** Onde residem todas as regras de validação do sistema, como checagem de estoque, regras de precificação e fluxo transacional de pedidos.
*   **Repositories (Camada de Acesso a Dados):** Camada responsável pela abstração e consulta das tabelas físicas do banco de dados utilizando queries do Diesel ORM.

---

## Tecnologias Utilizadas

### Backend
*   **Rust:** Linguagem com foco em segurança de memória e alta concorrência.
*   **Rocket Framework:** Framework web síncrono/assíncrono para construção de APIs REST robustas.
*   **Diesel ORM & SQLite:** Ferramentas para persistência relacional com tipagem estática e migrações estruturadas.
*   **Argon2id:** Algoritmo recomendado pelo OWASP para hash seguro de senhas.
*   **JWT (JSON Web Tokens):** Gerenciamento e expiração segura de sessões do usuário.

### Frontend
*   **React (Vite):** Biblioteca ágil para a construção da interface do usuário em Single Page Application (SPA).
*   **React Router Dom:** Gerenciamento de rotas públicas, privadas e administrativas.
*   **Axios:** Cliente HTTP com interceptors para injeção automática de tokens JWT.
*   **Context API:** Controle centralizado de estados para autenticação e carrinho de compras.

---

## Principais Funcionalidades

*   **Autenticação JWT:** Registro de novos usuários, login persistido localmente e rotas protegidas por validação de token no backend.
*   **Vitrine e Busca:** Filtros avançados por nome de produto e categorias com paginação performática direta no banco de dados.
*   **Carrinho de Compras:** Adição de itens, controle de quantidade baseada em estoque e soma de subtotais automática.
*   **Finalização de Compra (Checkout):** Integração transacional que valida o estoque atualizado, deduz as quantidades e fecha o pedido salvando o histórico de preços unitários.
*   **Painel Administrativo:** Área restrita para gerenciamento e cadastro completo de novas categorias e produtos.

---

## Como Instalar e Executar Localmente

### 1. Configurando o Backend
Acesse a pasta `backend`, crie um arquivo `.env` para carregar as configurações do ambiente e execute a aplicação:

```bash
cd backend
echo "DATABASE_URL=db.sqlite" > .env
echo "JWT_SECRET=chave_secreta_para_assinatura_do_token" >> .env
echo "JWT_EXPIRATION_HOURS=24" >> .env

cargo run
```
*As migrations do banco SQLite e os dados iniciais de teste (seeds) serão gerados e inseridos automaticamente no primeiro carregamento.*

### 2. Configurando o Frontend
Acesse a pasta `frontend`, instale as dependências com o gerenciador de pacotes e inicialize o servidor de desenvolvimento:

```bash
cd frontend
npm install
npm run dev
```

---

## Dados para Acesso Rápido
Durante a execução de desenvolvimento, você pode utilizar as seguintes credenciais previamente inseridas pelo banco:

*   **Acesso do Administrador:** `admin@shop.com` (Senha: `admin123`)
*   **Acesso do Cliente:** `teste@shop.com` (Senha: `teste123`)
