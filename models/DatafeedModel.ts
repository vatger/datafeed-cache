export type DatafeedModel = {
    general: DatafeedModelGeneral;
    pilots: Array<DatafeedModelPilot>;
    controllers: Array<DatafeedModelController>;
    atis: Array<DatafeedModelAtis>;
    servers: Array<DatafeedModelServer>;
    prefiles: Array<DatafeedModelPrefile>;
    facilities: Array<DatafeedModelFacility>;
    ratings: Array<DatafeedModelRating>;
    pilot_ratings: Array<DatafeedModelPilotRating>;
}

export type DatafeedModelGeneral = {
    version: number;
    reload: number;
    update: string;
    update_timestamp: string;
    connected_clients: number;
    unique_users: number;
}

export type DatafeedModelPilot = {
    cid: number;
    name: string;
    callsign: string;
    server: string;
    pilot_rating: number;
    latitude: number;
    longitude: number;
    altitude: number;
    groundspeed: number;
    transponder: string;
    heading: number;
    qnh_i_hg: number;
    qnh_mb: number;

    flight_plan: DatafeedModelPilotFlightPlan;

    logon_time: string;
    last_updated: string;
}

export type DatafeedModelPrefile = {
    cid: number;
    name: string;
    callsign: string;

    flight_plan: DatafeedModelPilotFlightPlan;

    last_updated: string;
}

type DatafeedModelPilotFlightPlan = {
    flight_rules: string;
    aircraft: string;
    aircraft_faa: string;
    aircraft_short: string;
    departure: string;
    arrival: string;
    alternate: string;
    cruise_tas: string;
    altitude: string;
    deptime: string;
    enroute_time: string;
    fuel_time: string;
    remarks: string;
    route: string;
    revision_id: number;
    assigned_transponder: string;
}

export type DatafeedModelController = {
    cid: number;
    name: string;
    callsign: string;
    frequency: string;
    facility: 0;
    rating: 1;
    server: string;
    visual_range: number;
    text_atis: Array<string> | null;
    last_updated: string;
    logon_time: string;
}

export type DatafeedModelAtis = {
    cid: number;
    name: string;
    callsign: string;
    frequency: string;
    facility: number;
    rating: number;
    server: string;
    visual_range: number;
    atis_code: string;
    text_atis: Array<string>;
    last_updated: string;
    logon_time: string;
}

export type DatafeedModelServer = {
    ident: string;
    hostname_or_ip: string;
    location: string;
    name: string;
    clients_connection_allowed: number;
    client_connections_allowed: boolean;
    is_sweatbox: boolean;
}

export type DatafeedModelFacility = {
    id: number;
    short: string;
    long: string;
}

export type DatafeedModelRating = {
    id: number;
    short: string;
    long: string;
}

export type DatafeedModelPilotRating = {
    id: number;
    short_name: string;
    long_name: string;
}