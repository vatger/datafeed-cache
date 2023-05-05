import {Router} from "express";
import DatafeedController from "./controller/DatafeedController";
import DatafeedVatgerController from "./controller/DatafeedVatgerController";

export const GlobalRouter = Router();

GlobalRouter.get("/datafeed", DatafeedController.getDatafeed);
GlobalRouter.get("/datafeed/general", DatafeedController.getDatafeedGeneral);
GlobalRouter.get("/datafeed/controllers", DatafeedController.getDatafeedControllers);
GlobalRouter.get("/datafeed/pilots", DatafeedController.getDatafeedPilots);
GlobalRouter.get("/datafeed/atis", DatafeedController.getDatafeedAtis);
GlobalRouter.get("/datafeed/servers", DatafeedController.getDatafeedServers);

// VATGER Specific Routes
GlobalRouter.get("/datafeed/controllers/ger", DatafeedVatgerController.getVatgerControllers);
GlobalRouter.get("/datafeed/pilots/ger", DatafeedVatgerController.getVatgerPilots);
GlobalRouter.get("/datafeed/atis/ger", DatafeedVatgerController.getVatgerAtis);