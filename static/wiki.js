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
        const { wiki } = await response.json();
        const ownId = location.pathname.match("([^/.]+)(\\.[^.]+)?$")[1];
        console.log({ ownId });
        const otherPages = wiki.filter(({ id }) => id !== ownId);

        popup.classList.remove("hashtag__popup--loading");

        if (otherPages.length === 0) {
          popup.classList.add("hashtag__popup--empty");

          popup.innerText = "No other pages found with this hashtag";
        } else {
          const ul = document.createElement("ul");

          ul.classList.add("hashtag__list");

          popup.classList.add("hashtag__popup--with-list");
          popup.innerHTML = "";
          popup.appendChild(ul);

          otherPages.forEach(({ id, title }) => {
            const li = document.createElement("li");
            const a = document.createElement("a");

            ul.appendChild(li);

            li.classList.add("hashtag__list-item");
            li.appendChild(a);

            a.innerText = title;
            a.href = `${id}.html`;
            a.classList.add("hashtag__list-link");
          });
        }
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
  toggle.addEventListener("mousedown", () =>
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
