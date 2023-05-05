import express from "express";
import {GlobalRouter} from "./routes";
import DatafeedService from "./service/DatafeedService";

const application = express();

setInterval(async () => {
    await DatafeedService.getDatafeed();
}, 1000 * 15);

application.listen(8000, "0.0.0.0", () => {
    console.log("Listening");
});

application.use("/", GlobalRouter);