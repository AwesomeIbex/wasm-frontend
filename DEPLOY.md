---

# Zero to Hero

> How to create your new app, modify it and deploy it - step by step guide.

## 1. Create a new project

I want to show you how to create, build and host your website for free, so we will need a public GitHub repository.

1. The simplest way how to do it is to click on the green button **Use this template** on the GitHub [profile](https://github.com/seed-rs/seed-quickstart-webpack) of this quickstart.

1. Make sure Git doesn't automatically convert your newlines to CLRF because linters don't like it.
    - Run `$ git config --global core.autocrlf` in your terminal and it should return `input` or `false`. See [Git docs](https://git-scm.com/book/en/v2/Customizing-Git-Git-Configuration) for more info.

1. Clone your new repository to your local machine. I use [GitKraken](https://www.gitkraken.com/), but you are probably better developer than me - use your favorite terminal.

## 2. Install / check required tools

1. Make sure you have basic tools installed:

   - [Yarn](https://yarnpkg.com/lang/en/docs/install) - run `$ yarn -v` in terminal. It should output something like `1.22.4`
   - [Node.js](https://nodejs.org) - `$ node -v` => `v12.13.0`
   - [Rust](https://www.rust-lang.org/tools/install) - `$ rustc -V` => `rustc 1.43.1 (8d69840ab 2020-05-04)`
   - Rust target `wasm` - `$ rustup target list` => `.. wasm32-unknown-unknown (installed) ..`
       - Install: `$ rustup target add wasm32-unknown-unknown`
       
1. Platform-specific tools like `ssl` and `pkg-config`:
    - Follow recommendations in build errors (during the next chapter).
    - _Note_: Don't hesitate to write a tutorial and create PR or write a Wiki page for your platform.

1. These tools are required by some commands:

   - [wasm-pack](https://rustwasm.github.io/wasm-pack/)

     - Check: `$ wasm-pack -V` => `wasm-pack 0.9.1`
     - Install: `$ cargo install wasm-pack`

   - [cargo-make](https://sagiegurari.github.io/cargo-make/)

     - Check: `$ cargo make -V` => `cargo-make 0.30.7`
     - Install: `$ cargo install cargo-make`

   - [nightly rustfmt](https://github.com/rust-lang/rustfmt#on-the-nightly-toolchain)
     - Check: `$ cargo +nightly fmt -- -V` => `rustfmt 1.4.14-nightly (a5cb5d26 2020-04-14)`
     - Install:
       1. `$ rustup toolchain install nightly`
       2. `$ rustup component add rustfmt --toolchain nightly`

## 3. Prepare your project for work

1. Open terminal in your project
1. Install Webpack and other dependencies - `$ yarn`
1. Try to start dev-server - `$ yarn start` - and then open [localhost:8000](http://localhost:8000) in a browser.
1. Stop server (try `Ctrl+c`).
1. Try to lint your project - `$ cargo make verify_only` - you shouldn't see any errors.
1. Modify files like `LICENCE`, `README.md` and `Cargo.toml` as you wish and push changes into GitHub.

## 4. Write your website

1. Open project in your favorite IDE. I use [IntelliJ](https://www.jetbrains.com/idea/download) or [VS Code](https://code.visualstudio.com/).
1. Run `$ yarn start` in terminal.
1. Open [localhost:8000](http://localhost:8000) in a browser.
1. You can open it also in your mobile phone:
   1. Make sure your dev-server aka computer is in the same network as your phone.
   1. Find out your local IP address. Use e.g. this online tool: https://www.whatismybrowser.com/detect/what-is-my-local-ip-address.
   1. Open URL with found IP address and default port (e.g. `192.168.1.5:8000`) on your phone.

_Note_: You don't have to follow all steps below - reuse starter project code as you need.

### Header

1. Open `src/page/partial/header.rs` in your IDE.
1. Delete function `header_visibility`.
1. Write a new body for function `view`.

### Footer

1. Open `src/page/partial/footer.rs` in your IDE.
1. Write a new body for function `view`.

### 404

1. Open `src/page/not_found.rs` in your IDE.
1. Write a new body for function `view`.

### Home page

1. Open `src/page/home.rs` in your IDE.
1. Write a new body for function `view`.

### About page

1. Open `src/page/about.rs` in your IDE.
1. Write a new body for function `view`.

### App core

1. Open `src/lib.rs` in your IDE.
1. Change `TITLE_SUFFIX` value.
1. Delete `MAIL_TO_KAVIK` and `MAIL_TO_HELLWEB`.
1. Write a new body for function `view`.

### Favicons

1. Delete or replace files in `/favicons`.
1. Open `static/templates/favicons.hbs` in your IDE.
1. Delete content or update it.
   - _Note_: Templates are written in [Handlebars](https://handlebarsjs.com/).

### Loading page

1. Open `static/templates/loading_page.hbs` in your IDE.
1. Delete content or update it.

### Social media & basic settings

1. Open `static/templates/social_media.hbs` in your IDE.
1. Delete content or update it.

### Basic HTML

1. Open `static/index.hbs` in your IDE.
1. Update content.

### Fonts

1. Delete or replace files and directories in `static/fonts`.
1. Open `/css/fonts.css` in your IDE.
1. Delete content or update it.

### Images & other files

1. Delete or replace files in `static/images`.
1. Delete `static/Martin_Kavik_resume.pdf`.

### TailwindCSS

1. Open `tailwind.config.js` in your IDE.
1. Update content or replace it with the default one:

```js
module.exports = {
  theme: {},
  variants: {},
  plugins: []
};
```

### Custom CSS

1. Open `/css/custom.css` in your IDE.
1. Delete content or update it.

## 5. Prepare your project for deploy

How to format, lint and test your project.

And how to setup Github Actions with deploy into Netlify.

### Formatter & Linter

1. Format: `$ cargo make fmt` (it overwrites files) or only `$ cargo make fmt_check`
1. You can modify format settings in:
   - `rustfmt.toml`
   - `Makefile.toml` - tasks `fmt` and `fmt_check`
1. Lint: `$ cargo make clippy`
1. You can modify linter settings in:
   - `Makefile.toml` - task `clippy`

### Testing

1. Run `$ cargo make test_h firefox` for headless testing in Firefox.
   - There are more similar commands - see `Makefile.toml`
   - _Note_: There is only one test in this project (`tests/test.rs`), see [seed-rs-realworld](https://github.com/seed-rs/seed-rs-realworld) for more examples.
1. If you want to test prerendered website:
   1. `$ yarn build:prerender`
   1. `$ yarn serve:dist`
   1. Open [localhost:8000](http://localhost:8000) in a browser.
   1. _Tip_: Always test it also in production environment because e.g. routing is a little bit different among servers.
1. **Always run `$ cargo make verify`** before push to make sure CI pipeline will accept your code.
   - It'll format your code, lint it and start headless tests in Firefox.
   - You can change its behaviour in `Makefile.tom` - task `verify` (similar task `verify_only` is used in CI).

### Netlify

1. Create a new [Netlify](https://www.netlify.com/) site.
1. _[Optional]_ Change site name or/and use your own domain.
1. _[Optional]_ Turn on HTTPS.
1. _[Optional]_ Add badge to project's `/README.md` (Site detail > `Settings` > `General` > `Status badges`).
1. Note somewhere **Site id** (Site detail > `Settings` > `General` > _API ID_)
1. Create and note somewhere **Access token** (Click on your avatar > `User settings` > `Applications` > `New access token` > Name it for instance `GitHub`)
1. _[Optional]_ Adjust `/netlify.toml` to suit your needs. [Netlify docs](https://www.netlify.com/docs/netlify-toml-reference/).

### Github Actions

1. Open your GitHub repository in your favorite browser.
1. Click on `Settings` and then on `Secrets`.
1. Add _secret_ `NETLIFY_SITE_ID` and set it's value to **Site id**.
1. Add _secret_ `NETLIFY_ACCESS_TOKEN` and set it's value to **Access token**.
1. Click on `Actions` in the side menu and make sure that `Actions` are enabled.
1. _[Optional]_ Modify `/.github/workflows/main.yml`.
   - Replace `yarn build:prerender` with `yarn build:release` if you don't want to prerender pages.
1. _[Optional]_ Push your code and switch to tab `Actions` to check that everything works.

### Manual Deployment

1. Run `yarn build:release`.
1. Check if build works by running `yarn serve:dist` and view in a browser.
1. Deploy the contents of the `dist` directory to a web server.