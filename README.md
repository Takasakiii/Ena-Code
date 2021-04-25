# Ena Code

É um pequeno trocador de perfil para Visual Studio Code, fazendo que possa segregar configs e extensões de acordo com o contexto/lang.

**Esse projeto ainda é um alfa, muitos bugs e coisas inacabadas, mas sua principal função é utilizavel.**


## Contrução e instalação:

### Dependencias:
 - [Rust](https://www.rust-lang.org/)
### Passos:
 - Na pasta do projeto, use o cargo para instalar o projeto:
 ```sh
 $ cargo install ecode
 ```

 ## Uso:
 ```sh
$ ecode [profile] [path]
```
> Sendo [profile] e [path] argumentos opcionais.

Ena Code usa {userfolder}/.ena-code para salvar os profiles e configurações
> Pasta userprofile pode ser trocada atraves da criação de um arquivo na userprofile chamado `.enarc` (esse arquivo usa o formato de codificação yaml) com o atributo `enaHomePath`.

Nas configurações é possivel mudar a pasta home dos profiles, para backup ou melhor localização e mudar o execultavel do vs code.
