import notification_system from "notification-system";

let scraped_json = await fetch("https://dtu.ac.in/")
    .then(response => {
        return response.text();
    })
    .then(notification_system.scrape)
    .then((info) => JSON.stringify(info, null, 2));

console.log(scraped_json);