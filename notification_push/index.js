import notification_system, { Configuration } from "notification-system";
import fs from "node:fs";
function save_request(data) {
    fetch("http://localhost:8080/api/v1/notification/save", {
        method: "POST",
        headers: {
            "Content-Type": "application/json",
        },
        body: JSON.stringify({ data: data, secret: process.env.NOTIF_SECRET }),
    })
        .then(response => {
        if (response.status != 200) {
            throw new Error("Failed to save request");
        }
        return response.json();
    })
        .then(data => console.log("Request saved", data))
        .catch(error => console.error(error));
}
let html = await fetch("https://dtu.ac.in/")
    .then(response => {
    return response.text();
});
let config = Configuration.default_config();
if (fs.existsSync("old_state.html")) {
    let old_html = fs.readFileSync("old_state.html").toString();
    let diff = notification_system.difference(html, old_html, config);
    console.log(JSON.stringify(diff, null, 2));
    if (diff.length > 0) {
        save_request(diff);
    }
}
else {
    let diff = [];
    console.log(diff);
}
fs.writeFileSync("old_state.html", html);
