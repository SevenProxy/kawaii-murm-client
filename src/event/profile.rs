//! # MIP-XX: Perfil de autor (kind 0) — exemplo passo a passo
//!
//! **Conceito (como no Nostr):** o perfil NÃO é um cadastro num banco central,
//! nem um arquivo salvo no client. O perfil é apenas **mais um evento assinado**
//! (kind 0), igual a um post. Quem guarda ele são os relays; o client só
//! "pergunta" os eventos kind 0 de cada autor e renderiza.
//!
//! Por isso o passo a passo inteiro aqui é:
//!   1. definir o formato do perfil (um struct JSON);
//!   2. transformar o perfil em um `Event` (com `content` = JSON do perfil);
//!   3. assinar como qualquer outro evento (`Event::sign`);
//!   4. publicar nos relays (fora do escopo deste arquivo);
//!   5. quando for ler: buscar kind 0 do autor no relay, validar (`verify`)
//!      e desserializar o `content` de volta para o struct.
//!
//! **Como ativar este módulo:** quando quiser usar, adicione UMA linha em
//! `src/event/mod.rs`:
//!
//! ```rust,ignore
//! pub mod profile;
//! ```
//!
//! Nada mais precisa mudar nos arquivos existentes — este arquivo só
//! reutiliza `Event`, `Payload`, `kinds`, `tags` e `Identity` já prontos.

use serde::{
    Deserialize,
    Serialize,
};

use crate::event::event::{
    Event,
    Payload,
};
use crate::event::kinds;
use crate::identity::Identity;

/// Formato do perfil de um autor.
///
/// Este struct é a "fotografia" do perfil. Cada campo vira uma chave do JSON
/// que será gravado no campo `content` do evento kind 0.
///
/// A regra do Nostr é: o relay NÃO entende estes campos; ele só guarda a
/// string. Quem interpreta é o client. Isso permite adicionar campos novos
/// sem mudar o protocolo — por isso usamos `Option` (campos podem faltar).
#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
pub struct Profile {
    /// Nome de exibição do autor.
    pub name: Option<String>,

    /// Biografia / descrição curta.
    pub about: Option<String>,

    /// URL da imagem de avatar (o relay não guarda a imagem, só a URL).
    pub picture: Option<String>,

    /// Identificador de verificação (ex: `nome@dominio.com`), como o NIP-05
    /// do Nostr. No murm isso ainda é um campo livre.
    pub nip05: Option<String>,
}

impl Profile {
    /// Cria um perfil vazio (todos os campos `None`).
    pub fn new() -> Self {
        Self::default()
    }

    // ------------------------------------------------------------------
    // PASSO 1: preencher o perfil (construtores encadeados)
    // ------------------------------------------------------------------

    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn about(mut self, about: impl Into<String>) -> Self {
        self.about = Some(about.into());
        self
    }

    pub fn picture(mut self, picture: impl Into<String>) -> Self {
        self.picture = Some(picture.into());
        self
    }

    pub fn nip05(mut self, nip05: impl Into<String>) -> Self {
        self.nip05 = Some(nip05.into());
        self
    }

    // ------------------------------------------------------------------
    // PASSO 2: perfil -> JSON compacto (vai para `content`)
    // ------------------------------------------------------------------

    /// Serializa o perfil como JSON compacto (sem espaços), no formato
    /// que será armazenado no campo `content` do evento kind 0.
    pub fn to_json(&self) -> String {
        serde_json::to_string(self).expect("profile is always serializable")
    }

    // ------------------------------------------------------------------
    // PASSO 3: perfil -> evento kind 0 (ainda NÃO assinado)
    // ------------------------------------------------------------------

    /// Monta um `Event` de kind 0 a partir do perfil.
    ///
    /// Repare que não há nada de especial aqui: é o mesmo `Event::new`
    /// usado para posts, só muda o `kind` e o `content` (que é o JSON).
    pub fn event(&self, identity: &Identity) -> Event {
        Event::new(
            identity.pubkey_hex(), // quem é o autor
            kinds::PROFILE,        // kind 0 = perfil
            vec![],                // kind 0 normalmente não precisa de tags
            self.to_json(),        // o perfil em si
        )
    }

    // ------------------------------------------------------------------
    // PASSO 4: perfil -> payload assinado, pronto para publicar
    // ------------------------------------------------------------------

    /// Cria o evento e assina em uma única chamada, devolvendo o
    /// `Payload` pronto para ser enviado ao relay.
    pub fn sign(&self, identity: &Identity) -> Payload {
        self.event(identity).sign(identity)
    }

    // ------------------------------------------------------------------
    // PASSO 5 (leitura): payload validado -> perfil
    // ------------------------------------------------------------------

    /// Tenta ler um `Profile` de um `Payload` recebido do relay.
    ///
    /// Retorna `None` se:
    ///   - o evento não for kind 0; ou
    ///   - a assinatura/id não passarem na validação (`verify`); ou
    ///   - o `content` não for um JSON válido do `Profile`.
    ///
    /// IMPORTANTE: sempre valide ANTES de confiar no conteúdo — o relay
    /// pode estar comprometido e qualquer pessoa pode tentar forjar
    /// o perfil de outra. O `verify` é o que garante a autoria.
    pub fn from_payload(payload: &Payload) -> Option<Profile> {
        if payload.kind != kinds::PROFILE {
            return None;
        }
        if !payload.verify() {
            return None;
        }
        serde_json::from_str(&payload.content).ok()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Fluxo completo de ESCREVER o perfil:
    /// montar -> assinar -> validar.
    #[test]
    fn sign_profile_roundtrip() {
        let identity = Identity::generate();

        let profile = Profile::new()
            .name("Fulano")
            .about("Engenheiro de software")
            .picture("https://example.com/avatar.png")
            .nip05("fulano@murm.example");

        let payload = profile.sign(&identity);

        assert_eq!(payload.kind, kinds::PROFILE);
        assert!(payload.verify(), "perfil assinado deve ser válido");
        assert!(payload.verify_with(&identity));
        assert!(payload.size_ok());
    }

    /// Fluxo completo de LER o perfil:
    /// receber do relay -> validar -> desserializar.
    #[test]
    fn read_profile_from_payload() {
        let identity = Identity::generate();

        let profile = Profile::new()
            .name("Cicrana")
            .about("Designer")
            .picture("https://example.com/avatar2.png");

        let payload = profile.sign(&identity);

        let read = Profile::from_payload(&payload)
            .expect("payload válido deve virar um perfil");
        assert_eq!(read, profile);
    }

    /// O `content` do evento é exatamente o JSON do perfil.
    #[test]
    fn content_is_profile_json() {
        let identity = Identity::generate();
        let profile = Profile::new().name("X").about("Y");

        let event = profile.event(&identity);

        assert_eq!(event.content, profile.to_json());
        assert_eq!(event.kind, kinds::PROFILE);
    }

    /// Rejeita payloads que não são kind 0.
    #[test]
    fn rejects_non_profile_kind() {
        let identity = Identity::generate();
        let post = crate::event::event::Event::new(
            identity.pubkey_hex(),
            kinds::POST,
            vec![crate::event::tags::topic("murm")],
            "oi".to_string(),
        )
        .sign(&identity);

        assert!(Profile::from_payload(&post).is_none());
    }

    /// Rejeita perfis adulterados (conteúdo trocado depois de assinado).
    #[test]
    fn rejects_tampered_profile() {
        let identity = Identity::generate();
        let profile = Profile::new().name("Original");

        let mut payload = profile.sign(&identity);
        payload.content = Profile::new().name("Hacker").to_json();

        assert!(Profile::from_payload(&payload).is_none());
    }

    /// Perfil sem nenhum campo ainda é um JSON válido e legível.
    #[test]
    fn empty_profile_roundtrip() {
        let identity = Identity::generate();
        let profile = Profile::new();

        let payload = profile.sign(&identity);
        let read = Profile::from_payload(&payload).expect("vazio ainda é válido");

        assert_eq!(read, Profile::new());
    }
}
