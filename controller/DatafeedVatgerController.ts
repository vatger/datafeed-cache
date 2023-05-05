import {Request, Response} from "express";
import pointInPolygon from "point-in-polygon";
import {DatafeedModel, DatafeedModelAtis, DatafeedModelController, DatafeedModelPilot} from "../models/DatafeedModel";
import DatafeedService from "../service/DatafeedService";

// Polygon representing Germany (low poly)
const germanyPolygon: Array<Array<number>> =
    [
        [47.610078, 7.476857],
        [48.971350, 8.189489],
        [49.466566, 6.380295],
        [51.028373, 5.932113],
        [54.843480, 7.145719],
        [54.642252, 14.133224],
        [50.843505, 14.750028],
        [50.249433, 12.140394],
        [48.675772, 13.854829],
        [47.475928, 12.926649]
    ];

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
        // TODO: Should this take too long in the future, we can bounds check lat & lon to a square over Germany.
        // Only if the plane is in this rect, we check if it's actually in the poly.
        return pointInPolygon([p.latitude, p.longitude], germanyPolygon);
    });

    response.send({
        count: pilots.length,
        pilots: pilots
    });
}

async function getVatgerAtis(request: Request, response: Response)
{
    const datafeed: DatafeedModel | null = await DatafeedService.getCachedDatafeed();
    if (datafeed == null)
    {
        response.status(500).send({});
        return;
    }

    const atis: DatafeedModelAtis[] = datafeed.atis.filter((a: DatafeedModelAtis) => {
        return ((a.callsign.startsWith("ED") || a.callsign.startsWith("ET")) && a.frequency != "199.998");
    });

    response.send(atis);
}

export default {
    getVatgerControllers,
    getVatgerPilots,
    getVatgerAtis
}