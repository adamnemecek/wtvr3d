import { LitElement, html, css } from 'lit-element';
import { classMap } from 'lit-html/directives/class-map.js';
import init from "../../../pkg/wtvr3d.js";
import {} from '../../lit-texture/lit-texture.js';

export class Wtvr3dDemo extends LitElement {
  static get properties() {
    return {
      page : { type : String },
      wasmReady : { type : Boolean }
    };
  }

  static get styles() {
    return css`
      :host {
        display : flex;
        height : 100%;
        flex-direction: row;
        align-items: center;
        justify-content: flex-start;
        font-size: calc(10px + 2vmin);
        color: #1a2b42;
      }

      nav {
        min-width : 250px;
        background: #fff;
        height : 100%;
      }

      nav ul {
        max-width : 200px;
        margin: 0 auto;
        padding: 0;
      }

      nav ul li {
        display: flex;
      }

      nav ul li a {
        color: #5a5c5e;
        text-decoration: none;
        font-size: 18px;
        line-height: 36px;
      }

      nav ul li a:hover,
      nav ul li a.active {
        color: blue;
      }

      main {
        flex-grow: 1;
      }

      main {
        height: 100%;
        width : 100%;
        display : flex;
        align-items : center;
        justify-content : center;
      }
      @media (max-width : 700px){
        :host {
          flex-direction : column;
        }
        nav {
          height : auto;
        }
      }
    `;
  }

  constructor() {
    super();
    this.page = 'default';
    this.wasmReady = false;
    this.initializeWasm();
    
  }

  async initializeWasm() {
    await init("../../../pkg/wtvr3d_bg.wasm");
    this.wasmReady = true;
  }

  render() {
    if(!this.wasmReady) {
      return html``;
    }
    return html`
      <nav>
        <ul>
          <li>
            <a href="#default" class=${this.__navClass('default')} @click=${this.__onNavClicked}>
              Default
            </a>
          </li>
          <li>
            <a href="#litTexture" class=${this.__navClass('litTexture')} @click=${this.__onNavClicked}>
              Lit Texture
            </a>
          </li>
        </ul>
  </nav>

      <main>
        ${this._renderPage()}
      </main>
    `;
  }

  _renderPage() {
    switch (this.page) {
      case 'default':
        return html`
          <h1 class="center">Wtvr3d Demos</h1>
        `;
      case 'litTexture':
        return html`
          <lit-texture></lit-texture>
        `;
      default:
        return html`
          <p>Page not found try going to <a href="#default">Default</a></p>
        `;
    }
  }

  __onNavClicked(ev) {
    ev.preventDefault();
    this.page = ev.target.hash.substring(1);
  }

  __navClass(page) {
    return classMap({ active: this.page === page });
  }
}
