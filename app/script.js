const ws = new WebSocket("ws://127.0.0.1:3030/ws");
const eventsList = document.getElementById("events");

ws.onmessage = (event) => {
  let msg = JSON.parse(event.data);
  msg.key = msg.key.replace("(", "_").replace(")", "_");
  console.log(msg, msg.name.charCodeAt(0));
  const svgDoc = document.getElementById("keyboard-svg").contentDocument;
  let svgKey = svgDoc.getElementById(msg.name.charCodeAt(0));
  if (!svgKey) {
    svgKey = svgDoc.getElementById(msg.key + msg.name);
  }
  if (!svgKey) {
    svgKey = svgDoc.getElementById(msg.name);
  }
  if (!svgKey) {
    svgKey = svgDoc.getElementById(msg.key);
  }
  if (!svgKey) {
    return;
  }
  const DELAY_MS = 300;
  svgKey.style.fill = "black";
  setTimeout(() => {
    svgKey.style.fill = "none";
  }, DELAY_MS);
};

ws.onopen = () => {
  console.log("WebSocket connection opened");
};

ws.onclose = () => {
  console.log("WebSocket connection closed");
};
