import notification_system from "notification-system";
import fs from "node:fs";

let html = await fetch("https://dtu.ac.in/")
    .then(response => {
        return response.text();
    });

let old_html = fs.readFileSync("old_state.html").toString();
let config = notification_system.Configuration.default_config();


let diff = notification_system.difference(html, old_html,config);

console.log(diff);

fs.writeFileSync("old_state.html", html);

fs.writeFileSync("diff.json", JSON.stringify(diff, null, 2));
