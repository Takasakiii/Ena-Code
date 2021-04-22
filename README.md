# Ena Code

É um pequeno trocador de perfil para Visual Studio Code, fazendo que possa segregar configs e extensões de acordo com o contexto/lang.

**Esse projeto ainda é um alfa, muitos bugs e coisas inacabadas, mas sua principal função é utilizavel.**

Bugs e Sugestões poderá ser enviadas nos Inssues, quer ajudar no projeto fique livre para enviar seus Pull Requests.


## Contrução e instalação:

### Dependencias:
 - [Rust](https://www.rust-lang.org/)
 - [Git](https://git-scm.com/)

### Passos:
 - Clone o repositorio:
```sh
$ git clone https://github.com/Takasakiii/Ena-Code.git
```
 - Na pasta do projeto, use o cargo para instalar o projeto:
 ```sh
 $ cargo install --path .
 ```

 ## Uso:
 ```sh
$ ecode [profile] [path]
```
> Sendo [profile] e [path] argumentos opcionais.

Ena Code usa {userfolder}/ena-code para salvar os profiles e configurações

Nas configurações é possivel mudar a pasta home dos profiles, para backup ou melhor localização e mudar o execultavel do vs code.