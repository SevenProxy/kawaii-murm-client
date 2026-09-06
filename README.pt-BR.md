# murm

O **murm** é um cliente descentralizado de mensagens e rede social com foco em privacidade. Ele usa uma arquitetura baseada em relays, inspirada no modelo do Nostr: toda mensagem é um *evento assinado* autenticado por um par de chaves Ed25519 que vive somente no seu dispositivo — não existe conta central, banco de dados em servidor ou senha.

Este repositório é o **cliente** de referência, escrito em Rust e compilado para WebAssembly com [Yew](https://yew.rs), e também inclui um binário nativo que funciona como um cliente de relay mínimo (MIP-03).

> Leia em [English](README.md)

---

## Sumário

- [Destaques](#destaques)
- [Como funciona](#como-funciona)
- [O protocolo murm](#o-protocolo-murm)
- [Stack de tecnologias](#stack-de-tecnologias)
- [Roadmap](#roadmap)
- [Começando](#começando)
- [Estrutura do projeto](#estrutura-do-projeto)
- [Contribuindo](#contribuindo)
- [Contribuidores](#contribuidores)

## Destaques

- **Identidade soberana** — um par de chaves Ed25519 é gerado localmente no navegador e armazenado no `localStorage`. Sua chave privada nunca sai do seu dispositivo.
- **Conteúdo assinado criptograficamente** — todo evento (post, perfil, reação) é hasheado e assinado. Qualquer adulteração é detectada na verificação local.
- **Sem intermediários confiáveis** — relays são tratados como armazenamento não confiável; as assinaturas são validadas no cliente (MIP-05).
- **Web + nativo** — o mesmo código gera um app web responsivo (WASM) e um binário nativo com cliente de relay para publish/fetch/scan.
- **UX estilo Discord** — canais, tema escuro e editor de perfil dentro do app.

## Como funciona

1. Na primeira visita, o cliente gera um par de chaves Ed25519 e o persiste no navegador.
2. Você publica *eventos assinados* — posts, respostas, reações ou dados de perfil — na rede.
3. Os relays armazenam e servem esses eventos para qualquer cliente que os solicitar.
4. Todo cliente verifica IDs e assinaturas localmente antes de confiar em qualquer conteúdo.

Como o conteúdo vive nos relays e é assinado pela sua chave, você pode trocar de cliente ou de relay sem nunca perder a sua identidade.

## O protocolo murm

O `murm` define seu próprio conjunto de propostas de melhoria (MIPs) que moldam eventos, IDs e a comunicação com relays:

| MIP | O que define |
| --- | --- |
| MIP-01 | Formato do evento e o limite de tamanho de 2 MiB |
| MIP-02 | Payload canônico, IDs de evento SHA-256 e assinaturas Ed25519 |
| MIP-03 | Binding HTTP de relay — endpoints `submit`, `fetch` e `scan` |
| MIP-04 | Kinds iniciais de evento: `profile (0)`, `post (1)`, `comment (2)`, `reaction (3)` |
| MIP-05 | Relays não são confiáveis — o cliente deve sempre verificar localmente |

## Stack de tecnologias

### Em uso atualmente

**Núcleo do cliente**
- [Rust](https://www.rust-lang.org/) (edition 2024)
- [Yew](https://yew.rs/) `0.23` — framework web reativo (CSR), function components
- [yew-router](https://docs.rs/yew-router) — roteamento client-side (`/`, `/generate`, `/app`)
- [Trunk](https://trunkrs.dev/) — build de WASM e servidor de desenvolvimento
- [Tailwind CSS](https://tailwindcss.com/) (via CDN) + CSS customizado (Inter / JetBrains Mono)

**Criptografia e identidade**
- [ed25519-dalek](https://docs.rs/ed25519-dalek) — assinatura/verificação Ed25519 para identidades
- [sha2](https://docs.rs/sha2) — digest SHA-256 para IDs de evento (MIP-02)
- [rand](https://docs.rs/rand) — geração de chaves com CSPRNG
- [hex](https://docs.rs/hex) — codificação hex de chaves, IDs e assinaturas

**Plataforma web**
- [gloo-storage](https://docs.rs/gloo-storage) — persistência em `localStorage` da identidade e do perfil
- [web-sys](https://docs.rs/web-sys) — bindings de DOM para inputs e textareas

**Serialização e protocolo**
- [serde](https://serde.rs/) / [serde_json](https://docs.rs/serde_json) — eventos JSON e serialização canônica
- [anyhow](https://docs.rs/anyhow) — tratamento ergonômico de erros

**Lado nativo (target não-WASM)**
- [reqwest](https://docs.rs/reqwest) — cliente HTTP assíncrono de relay (MIP-03)
- [tokio](https://tokio.rs/) — runtime assíncrono
- [clap](https://docs.rs/clap) — parsing de argumentos de CLI

## Roadmap

Próximos passos planejados para o cliente:

- [ ] Ligar publish/scan de relays ao build do navegador (conectar o cliente WASM ao cliente de relay MIP-03 via HTTP/WebSocket)
- [ ] Timeline funcional: ler posts dos relays e renderizá-los
- [ ] Threads — respostas e comentários conectados às tags `root` / `parent`
- [ ] UI de reações usando o kind `reaction`
- [ ] Mensagens diretas com criptografia ponta a ponta (novo kind de evento)
- [ ] Verificação de identidade estilo NIP-05 (indicador `nome@dominio`)
- [ ] Suporte a múltiplos relays com failover automático
- [ ] Notificações
- [ ] Bindings MIP-03 mais amplos (transporte WebSocket)
- [ ] Suíte de testes expandida e CI

## Começando

### Pré-requisitos

- Rust toolchain (stable)
- Target `wasm32-unknown-unknown`
- [Trunk](https://trunkrs.dev/)

```bash
rustup target add wasm32-unknown-unknown
cargo install trunk
```

### Rodar o app web

```bash
trunk serve
```

Abra `http://localhost:8080`.

### Binário nativo e testes

O crate também compila para o host com um entrypoint nativo que exercita a assinatura de eventos e o cliente de relay:

```bash
# roda a demo nativa (cria/carrega ./.murm_id e assina um evento de exemplo)
cargo run

# roda a suíte de testes (criptografia, eventos, perfis)
cargo test
```

## Estrutura do projeto

```
src/
├── config/     # configuração do cliente (URL padrão do relay, limites de scan)
├── event/      # primitivas do protocolo MIP
│   ├── event.rs    # Event + Payload, payload canônico, assinatura e verificação
│   ├── filters.rs  # filtros de evento MIP-03
│   ├── kinds.rs    # kinds de evento MIP-04
│   ├── tags.rs     # helpers de tags (root, parent, target, topic, lang)
│   └── profile.rs  # perfis de autor (kind 0)
├── identity/   # identidade Ed25519 (gerar, carregar, salvar, assinar)
├── relay/      # cliente de relay MIP-03 (submit / fetch / scan)
└── web/        # frontend Yew
    ├── components/  # UI reutilizável (fundo mesh)
    ├── pages/       # visões home, gerar-identidade e app
    └── router.rs    # definição de rotas
```

## Contribuindo

Contribuições são bem-vindas. Abra uma issue ou um pull request e siga as convenções de código existentes — o codebase é organizado em torno dos módulos MIP e mantém a lógica de protocolo independente da UI.

## Contribuidores

- [SevenProxy](https://github.com/SevenProxy) — autor e mantenedor