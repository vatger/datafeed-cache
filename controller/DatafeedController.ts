import { Request, Response } from 'express';
import DatafeedService from '../service/DatafeedService';

async function getDatafeed(request: Request, response: Response) {
    const datafeed = await DatafeedService.getCachedDatafeed();
    const failed = DatafeedService.getUpdateFailed();

    response.send({
        data: datafeed,
        failed: failed,
    });
}

async function getDatafeedGeneral(request: Request, response: Response) {
    const datafeed = await DatafeedService.getCachedDatafeed();
    const failed = DatafeedService.getUpdateFailed();

    response.send({
        data: datafeed?.general,
        controller_length: datafeed?.controllers.length,
        pilots_length: datafeed?.pilots.length,
        atis_length: datafeed?.atis.length,
        failed: failed,
    });
}

async function getDatafeedControllers(request: Request, response: Response) {
    const datafeed = await DatafeedService.getCachedDatafeed();
    const failed = DatafeedService.getUpdateFailed();

    response.send({
        data: datafeed?.controllers,
        length: datafeed?.controllers.length,
        failed: failed,
    });
}

async function getDatafeedPilots(request: Request, response: Response) {
    const datafeed = await DatafeedService.getCachedDatafeed();
    const failed = DatafeedService.getUpdateFailed();

    response.send({
        data: datafeed?.pilots,
        length: datafeed?.pilots.length,
        failed: failed,
    });
}

async function getDatafeedAtis(request: Request, response: Response) {
    const datafeed = await DatafeedService.getCachedDatafeed();
    const failed = DatafeedService.getUpdateFailed();

    response.send({
        data: datafeed?.atis,
        length: datafeed?.atis.length,
        failed: failed,
    });
}

async function getDatafeedServers(request: Request, response: Response) {
    const datafeed = await DatafeedService.getCachedDatafeed();
    const failed = DatafeedService.getUpdateFailed();

    response.send({
        data: datafeed?.servers,
        length: datafeed?.servers.length,
        failed: failed,
    });
}

async function getDatafeedPilotRatings(request: Request, response: Response) {
    const datafeed = await DatafeedService.getCachedDatafeed();
    const failed = DatafeedService.getUpdateFailed();

    response.send({
        data: datafeed?.pilot_ratings,
        length: datafeed?.pilot_ratings.length,
        failed: failed,
    });
}

async function getDatafeedMilitaryRatings(
    request: Request,
    response: Response
) {
    const datafeed = await DatafeedService.getCachedDatafeed();
    const failed = DatafeedService.getUpdateFailed();

    response.send({
        data: datafeed?.military_ratings,
        length: datafeed?.pilot_ratings.length,
        failed: failed,
    });
}

export default {
    getDatafeed,
    getDatafeedGeneral,
    getDatafeedControllers,
    getDatafeedPilots,
    getDatafeedAtis,
    getDatafeedServers,
    getDatafeedPilotRatings,
    getDatafeedMilitaryRatings,
};
