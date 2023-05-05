import {Request, Response} from "express";
import DatafeedService from "../service/DatafeedService";
import {DatafeedModel} from "../models/DatafeedModel";

async function getDatafeed(request: Request, response: Response)
{
    const datafeed = await DatafeedService.getCachedDatafeed();

    response.send(datafeed);
}

async function getDatafeedGeneral(request: Request, response: Response)
{
    const datafeed = await DatafeedService.getCachedDatafeed();

    response.send(datafeed?.general);
}

async function getDatafeedControllers(request: Request, response: Response)
{
    const datafeed = await DatafeedService.getCachedDatafeed();

    response.send(datafeed?.controllers);
}

async function getDatafeedPilots(request: Request, response: Response)
{
    const datafeed = await DatafeedService.getCachedDatafeed();

    response.send(datafeed?.pilots);
}

async function getDatafeedAtis(request: Request, response: Response)
{
    const datafeed = await DatafeedService.getCachedDatafeed();

    response.send(datafeed?.atis);
}

async function getDatafeedServers(request: Request, response: Response)
{
    const datafeed = await DatafeedService.getCachedDatafeed();

    response.send(datafeed?.servers);
}


export default {
    getDatafeed,
    getDatafeedGeneral,
    getDatafeedControllers,
    getDatafeedPilots,
    getDatafeedAtis,
    getDatafeedServers
}