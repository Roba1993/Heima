import './store';
import './room';
import './device';
import './slider';

import './devices/light';
import './devices/dim-light';
import './devices/socket';
import './devices/blind';
import './devices/meter';

import 'live-icons';
window.registerInteractiveIcons()


class HeimaUI extends HTMLElement {
    connectedCallback() {
        this.attachShadow({ mode: 'open' });
        this.render();

        store.addListener(this, null, this.render.bind(this));
    }

    disconnectedCallback() {
        store.removeListener(this);
    }

    render() {
        var html = '';

        store.get_rooms().forEach(room => {
            html += `<heima-room name="${room}" ></heima-room>`;
        });

        this.shadowRoot.innerHTML = html;
    }
}
customElements.define("heima-ui", HeimaUI);