import { putWake } from "./api";

const macAddressRegex = RegExp("^([0-9A-Fa-f]{2}[:-]){5}([0-9A-Fa-f]{2})|([0-9a-fA-F]{4}\\.[0-9a-fA-F]{4}\\.[0-9a-fA-F]{4})$");

export const handleResponse = (code: number, identifier: string) => {

  switch (code) {
    case 200:
      alert(`Successfully sent magic packet to ${identifier}.`);
      break;
    case 500:
      alert(`Something went wrong trying to wake up "${identifier}". Did you type the MAC address correctly?`);
      break;
    default:
      alert(`Something unexpected happened. Please try again.`);
      break;
  }
}

export const wakeUp = async (address: string, name: string | undefined = undefined) => {

  if (!macAddressRegex.test(address)) {
    alert("Error: Invalid MAC address, please try again.");
    return;
  }

  const result = await putWake(address);
  handleResponse(result, name || address);
}