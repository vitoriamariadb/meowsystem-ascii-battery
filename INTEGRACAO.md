# Integração com o MeowSystem

O que o MeowSystem precisa saber para compilar, instalar, configurar e pôr as cores do pack no
**Bateria em ASCII**.

| | |
| --- | --- |
| repositório | `https://github.com/vitoriamariadb/meowsystem-ascii-battery` (ramo `main`) |
| upstream | `https://github.com/pewmoe/Cosmic-Desktop-ASCII-Battery-Applet` (base: `195f2ca`, main, 1.3.0) |
| app id | `io.github.meowsystem.AsciiBattery` |
| binário | `meowsystem-ascii-battery` |
| nome no painel | «Bateria em ASCII» |
| licença | MPL-2.0, a do upstream |

## Compilar

Rust (testado com 1.99.1; este crate é edição 2021) e as bibliotecas de desenvolvimento do COSMIC (no
Pop!_OS: `build-essential`, `pkgconf`, `libxkbcommon-dev`, `libwayland-dev`). O
libcosmic vem pinado no `Cargo.lock` (git `c003a58f`); com `--locked` a compilação de
amanhã é a de hoje. O MeowSystem reaponta o libcosmic para o do painel instalado quando ele
compila contra o dele, e cai neste pino se não compilar.

```sh
cargo build --release --locked
cargo test --release --locked
```

## Instalar (em `~/.local`, sem root)

`just install-user` faz isto:

| arquivo | destino |
| --- | --- |
| `target/release/meowsystem-ascii-battery` | `~/.local/bin/meowsystem-ascii-battery` |
| `resources/io.github.meowsystem.AsciiBattery.desktop`, com `Exec=` trocado pelo caminho absoluto do binário | `~/.local/share/applications/io.github.meowsystem.AsciiBattery.desktop` |
| `resources/io.github.meowsystem.AsciiBattery.svg` | `~/.local/share/icons/hicolor/scalable/apps/` |
| `resources/io.github.meowsystem.AsciiBattery.metainfo.xml` | `~/.local/share/metainfo/` |

O painel acha o applet pelo nome do `.desktop` (`X-CosmicApplet=true`) e só relê a lista ao subir.
Um applet já posto no painel só troca de binário quando o `cosmic-panel` reinicia.

Desfazer: `just uninstall-user`. As escolhas do popup ficam em `$XDG_CONFIG_HOME/io.github.meowsystem.AsciiBattery/config.json` (`~/.config/…` sem a variável), um JSON do próprio applet, não do `cosmic-config`.

A configuração do applet antigo (`~/.config/com.github.pewmoe.cosmic-ext-ASCII-dot-battery/config.json`) não é migrada: o padrão vale e a pessoa refaz as escolhas. O `.desktop` instalado é o de `resources/`, sem o passo do `xdgen` do `build.rs` (que só serve ao flatpak).

## Extensão de tema

Sem extensão, tudo segue o tema do COSMIC. Com ela, o pack põe as cores por cima.

- **Arquivo:** `$XDG_CONFIG_HOME/meowsystem/extensoes/io.github.meowsystem.AsciiBattery/tema.conf`
  (`~/.config/…` sem `XDG_CONFIG_HOME`). A variável `MEOWSYSTEM_TEMA` aponta outro arquivo.
- **Quando é lido:** ao subir e a cada vez que o popup abre. Trocar de pack vale no próximo clique
  no botão; o painel não precisa reiniciar.
- **Formato:** uma `chave = valor` por linha, `#` começa comentário, espaço em volta é ignorado.
  Cor é `#rrggbb` ou `#rrggbbaa`; nome de paleta (`mauve`) quem traduz é o MeowSystem.
- **Falhas:** arquivo ausente é o tema do COSMIC; arquivo ilegível ou linha ruim (cor inválida,
  chave desconhecida) vira aviso no log e as outras linhas valem. Nada disso derruba o applet.

| chave | valor | onde aparece |
| --- | --- | --- |
| `acento` | cor | botões principais, escolhas marcadas e interruptores ligados |
| `destaque` | cor | texto e ícones sobre o acento |
| `fundo` | cor | fundo do popup |
| `borda` | cor | contorno e divisórias do popup |
| `texto` | cor | texto e ícones do popup |
| `bloco` | cor | fundo dos botões, dos campos e das linhas do popup |
| `tinta` | cor | a barra de blocos na barra do painel e a cor de apoio do popup, enquanto «usar o acento do sistema» está ligado; uma cor escolhida à mão no popup vale mais |

Exemplo (Catppuccin Mocha, acento mauve):

```ini
# gerado pelo MeowSystem a partir do pack
acento = #cba6f7
destaque = #1e1e2e
fundo = #1e1e2ef0
borda = #45475a
texto = #cdd6f4
bloco = #313244
tinta = #cdd6f4
```

A mesma convenção (pasta, formato, chaves de cor) vale para a Bandeja
(`io.github.meowsystem.Bandeja`) e para o Ajustes rápidos
(`io.github.meowsystem.AjustesRapidos`); o MeowSystem gera todos com o mesmo código.

## O que o MeowSystem faz pela seção «Extensões»

1. **Pino.** `assets/extensoes/ascii-battery.pino`: `URL=https://github.com/vitoriamariadb/meowsystem-ascii-battery.git`,
   `COMMIT=<hash do main>`, `LIBCOSMIC=c003a58f…`, `PADRAO=sim`.
2. **Build.** `scripts/extensoes_build.sh` clona em `~/.local/state/meowsystem/extensoes/ascii-battery/`,
   põe o commit do pino, roda `cargo build --release --locked`, carimba o commit para a segunda
   passada não recompilar e copia a tabela de «Instalar».
3. **Tema.** Na troca de pack escreve o `tema.conf` acima a partir da paleta.
4. **Painel.** Troca a entrada `com.github.pewmoe.cosmic-ext-ASCII-dot-battery` por `io.github.meowsystem.AsciiBattery` quando a pessoa aceita
   (a ordem dos applets é dela) e reinicia o `cosmic-panel`.
5. **Desinstalar.** `just uninstall-user` mais a pasta do clone; a configuração fica.
