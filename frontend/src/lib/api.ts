import { dev } from "$app/environment"

const getApiURL = () => {
  return (dev) ? "http://localhost:3000/api" : "/api"
}

export const getData = async () => {
  const request = await fetch(`${getApiURL()}/data`, {
    method: "GET"
  });

  const json = await request.json();
  return json
}

export const putWake = async (address: string) => {
  const request = await fetch(`${getApiURL()}/wake/${address}`, {
    method: "PUT"
  });

  const response = await request;
  return response.status;
}

