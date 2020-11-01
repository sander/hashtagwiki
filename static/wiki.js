const openPopup = tag => {
  alert("Placeholder popup for " + tag);
};

document.querySelectorAll("span[property='dc:references']").forEach(el =>
  el.addEventListener("click", () => openPopup(el.innerText)));
