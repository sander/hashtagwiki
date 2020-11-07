/**
 * @param {Object} toggle
 * @param {Element} toggle.wrapper
 * @param {Element} toggle.popup
 * @param {string} toggle.name
 */
const toggleHashtagPopup = async ({ wrapper, popup, name }) => {
  const isOpen = wrapper.classList.contains("hashtag--open");
  if (isOpen) {
    wrapper.classList.remove("hashtag--open");
    wrapper.classList.add("hashtag--closed");
  } else {
    const isFresh = popup.classList.contains("hashtag__popup--fresh");

    wrapper.classList.remove("hashtag--closed");
    wrapper.classList.add("hashtag--open");

    if (isFresh) {
      popup.classList.remove("hashtag__popup--fresh");
      popup.classList.add("hashtag__popup--loading");

      const response = await fetch(`../hashtag/${name.substr(1)}.json`);
      if (response.status === 200) {
        const json = await response.json();
        popup.innerText = JSON.stringify(json, null, 2);
      } else {
        popup.classList.remove("hashtag__popup--loading");
        popup.classList.add("hashtag__popup--error");
        popup.innerText = `Error loading hashtag (${response.status})`;
      }
    }
  }
};

/**
 * @param {Element} toggle
 */
const prepareHashtag = (toggle) => {
  const wrapper = document.createElement("span");
  const popup = document.createElement("span");

  toggle.classList.add("hashtag__toggle");
  toggle.parentNode.insertBefore(wrapper, toggle);
  toggle.addEventListener("click", () =>
    toggleHashtagPopup({ wrapper, popup, name: toggle.innerText })
  );

  wrapper.classList.add("hashtag");
  wrapper.classList.add("hashtag--closed");
  wrapper.appendChild(toggle);
  wrapper.appendChild(popup);

  popup.classList.add("hashtag__popup");
  popup.classList.add("hashtag__popup--fresh");
  popup.innerText = "Loading…";
};

document
  .querySelectorAll("span[property='dc:references']")
  .forEach(prepareHashtag);
