const macAddressRegex = /^([0-9A-Fa-f]{2}[:-]){5}([0-9A-Fa-f]{2})$/;

const handleResponse = (code, name) => {
  switch(code) {
    case 200:
      alert(`Successfully sent magic packet to ${name}.`)
      break;
    case 500:
      alert(`Something went wrong whilst trying to send magic packet to "${name}". Is the MAC address set correctly?`)
      break
    default:
      alert("Something unexpected happened. Please try again.")
      break;
  }
}

const wake = (macAddress, name = undefined) => {

  if (!macAddressRegex.test(macAddress)) {
    handleResponse(500, macAddress);
    return;
  }

  fetch(`/wake/${macAddress}`, {
    method: "PUT",
    headers: { "Content-Type": "application/json" }
  })
  .then(result => handleResponse(result.status, name ?? macAddress))
  .catch(err => console.log(error));
}

document.querySelector("#form").addEventListener("submit", (e) => {
  e.preventDefault()

  let formData = new FormData(e.target);
  let formProps = Object.fromEntries(formData);

  let address = formProps["address"]
  wake(address)
})

document.querySelectorAll("#device").forEach(device => {
  device.addEventListener("click", (e) => {
    e.stopPropagation();
    e.preventDefault();

    let name = e.target.getAttribute("data-name")
    let address = e.target.getAttribute("data-addr")

    console.log(`${name} -- ${address}`)

    if (name !== null && address !== null) {
      wake(address, name)
    }
  })
})