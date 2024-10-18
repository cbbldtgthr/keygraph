const ws = new WebSocket("ws://127.0.0.1:3030/ws");
const eventsList = document.getElementById("events");

ws.onmessage = (event) => {
  let msg = JSON.parse(event.data);
  console.log(msg);
  // Get the SVG element by ID
  const svgDoc = document.getElementById("keyboard-svg").contentDocument;
  let svgKey = svgDoc.getElementById(msg.key);
  if (!svgKey) {
    svgKey = svgDoc.getElementById(msg.name);
    // console.log(msg.name);
  }
  if (!svgKey) {
    svgKey = svgDoc.getElementById(msg.key + msg.name);
    // console.log(msg.key + msg.name);
  }
  if (!svgKey) {
    svgKey = svgDoc.getElementById(msg.name.charCodeAt(0));
  }
  if (!svgKey) {
    return;
  }
  // Update the fill property
  svgKey.style.fill = "green";

  setTimeout(() => {
    svgKey.style.fill = "none";
  }, 100);
};

ws.onopen = () => {
  console.log("WebSocket connection opened");
};

ws.onclose = () => {
  console.log("WebSocket connection closed");
};
