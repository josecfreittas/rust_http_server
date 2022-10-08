const delay = ms => new Promise(res => setTimeout(res, ms));

const showText = async (target, message, index, interval) => {
    for (letter in message) {
        target.innerHTML += `<span>${message[index++]}</span>`;
    }

    for (child of target.children) {
        child.classList.add("reveal");
        await delay(interval);
    }
}

const complementaryText = document.querySelector("h3");
complementaryText.innerHTML = "";

const mainText = document.querySelector("h1");
mainText.style = "opacity: 0";

(async () => {
    await showText(complementaryText, "static page served with", 0, 75);
    mainText.style = "opacity: 1";
})();
