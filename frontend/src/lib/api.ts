export const getData = async () => {
  const request = await fetch("/api/data", {
    method: "GET"
  });

  const json = await request.json();
  return json
}

export const putWake = async (address: string) => {
  const request = await fetch(`/api/wake/${address}`, {
    method: "PUT"
  });

  const response = await request;
  return response.status;
}

