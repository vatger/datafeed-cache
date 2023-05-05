import {Request, Response} from "express";
import pointInPolygon from "point-in-polygon";
import {DatafeedModel, DatafeedModelController, DatafeedModelPilot} from "../models/DatafeedModel";
import DatafeedService from "../service/DatafeedService";
import {germanyPolygon} from "../types/Polygon";

async function getVatgerControllers(request: Request, response: Response)
{
    const datafeed: DatafeedModel | null = await DatafeedService.getCachedDatafeed();
    if (datafeed == null)
    {
        response.status(500).send({});
        return;
    }

    const atc: DatafeedModelController[] = datafeed.controllers.filter((c: DatafeedModelController) => {
        return ((c.callsign.startsWith("ED") || c.callsign.startsWith("ET")) && c.frequency != "199.998");
    });

    response.send(atc);
}

async function getVatgerPilots(request: Request, response: Response)
{
    const datafeed: DatafeedModel | null = await DatafeedService.getCachedDatafeed();
    if (datafeed == null)
    {
        response.status(500).send({});
        return;
    }

    const pilots: DatafeedModelPilot[] = datafeed.pilots.filter((p: DatafeedModelPilot) => {
        return pointInPolygon([p.latitude, p.longitude], germanyPolygon);
    });

    response.send({
        count: pilots.length,
        pilots: pilots
    });
}

export default {
    getVatgerControllers,
    getVatgerPilots
}