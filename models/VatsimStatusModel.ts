export type VatsimStatusModel = {
    data: {
        v3: Array<string>;
        transceivers: Array<string>;
        servers: Array<string>;
        servers_sweatbox: Array<string>;
        servers_all: Array<string>;
    };
    user: Array<string>;
    metar: Array<string>;
};
