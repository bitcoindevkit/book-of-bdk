const tabSync = () => {
  const tabs = document.querySelectorAll(".tabbed-set > input")
  for (const tab of tabs) {
    tab.addEventListener("click", () => {
      const current = document.querySelector(`label[for=${tab.id}]`)
      const pos = current.getBoundingClientRect().top
      const labelContent = current.childNodes[0].innerHTML ? current.childNodes[0].innerHTML : current.innerHTML
      const labels = document.querySelectorAll('.tabbed-set > label, .tabbed-alternate > .tabbed-labels > label')
      for (const label of labels) {
        const thisLabelContent = label.childNodes[0].innerHTML ? label.childNodes[0].innerHTML : label.innerHTML

        if (thisLabelContent === labelContent) {
          document.querySelector(`input[id=${label.getAttribute('for')}]`).checked = true
        }
      }

      // Preserve scroll position
      const delta = (current.getBoundingClientRect().top) - pos
      window.scrollBy(0, delta)
    })
  }
}
window.addEventListener("load", (event) => {
  tabSync()
});
