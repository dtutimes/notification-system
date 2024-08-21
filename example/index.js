import notification_system from "notification-system";
import fs from "fs";

let html = await fetch("https://dtu.ac.in/")
    .then(response => {
        return response.text();
    });

let scraped_json = notification_system.scrape(html);

console.log(JSON.stringify(scraped_json, null, 2));


let old_html = fs.readFileSync("old_state.html").toString();
let diff = notification_system.difference(html, old_html);

fs.writeFileSync("old_state.html", html);

console.log(JSON.stringify(diff, null, 2));
