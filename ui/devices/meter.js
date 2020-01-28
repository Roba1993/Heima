import Toucher from './../toucher';

class HeimaMeter extends HTMLElement {
    connectedCallback() {
        this.device = store.get_device(this.getAttribute("id"));

        // create shadow dom
        this.attachShadow({ mode: 'open' });

        // create and add the icon and toucher
        this.toucher = new Toucher();
        this.toucher.innerHTML = `<live-icon-meter value="${this.device.status["Meter"].power}" max="${this.device.status["Meter"].max}"></live-icon-meter>`;
        this.shadowRoot.appendChild(this.toucher);

        this.device.addListener(this, null, this.update.bind(this));
    }

    disconnectedCallback() {
        this.device.removeListener(this);
    }

    update() {
        let icon = this.toucher.querySelector("live-icon-meter");
        icon.setAttribute("value", this.device.status["Meter"].power);
        icon.setAttribute("max", this.device.status["Meter"].max);
    }
}
customElements.define("heima-meter", HeimaMeter);