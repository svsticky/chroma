import {server} from "@/generated/server";
import * as pb_1 from "google-protobuf";
import {Storage} from "@/api";

/**
 * This module contains HTTP abstractions
 * to make life easier.
 *
 * The request and response payload formats are `application/protobuf`
 */
export namespace Http {
    /**
     * Send a HTTP request
     * @param path The path
     * @param method The HTTP method, must be valid according to the Fetch API
     * @param body Optional, the body to send
     * @param sessionId Optional, provide the session ID. If this is left blank,
     * the session ID will be retrieved from local storage
     */
    async function send<T extends pb_1.Message>(path: string, method: string = 'POST', body: pb_1.Message | null = null, sessionId: string | null = null): Promise<Response> {
        let bodyInner = null;
        if(body != null) {
            bodyInner = body.serializeBinary();
        }

        return await fetch(`${server}${path}`, {
            method: method,
            body: bodyInner,
            headers: {
                'Content-Type': 'application/protobuf',
                'Accept': 'application/protobuf',
                'Authorization': sessionId ?? Storage.getSessionId() ?? '',
            }
        });
    }

    /**
     * Parse the response body, if applicable.
     * The body will be parsed if and only if:
     * - The `Content-Length` header is returned
     * - The value of the `Content-Length` header is greater than `0`
     * - The status code indicates success (Status code lies in the range `200-299`)
     * - The status code was not `HTTP 204` (No Content)
     *
     * If one of the conditions is not met, the response provided in `r` is returned unchanged.
     * @param r The response to parse the body from
     * @param ty The type of the response body
     */
    async function decodeBodyMaybe<T extends pb_1.Message>(r: Response, ty: typeof pb_1.Message): Promise<T | Response> {
        if(r.ok && r.status != 204) {
            let contentLength = r.headers.get('content-length');
            if (contentLength != null && Number.parseInt(contentLength) > 0) {
                return <T>ty.deserializeBinary(new Uint8Array(await r.arrayBuffer()));
            }
        }

        return r;
    }

    /**
     * Send a GET request, ignoring the response body
     * @param path The path
     * @param sessionId Optional, provide the session ID. If this is left blank,
     * the session ID will be retrieved from local storage
     */
    export async function get(path: string, sessionId: string | null): Promise<Response> {
        return await send(path, 'GET', null, sessionId);
    }

    /**
     * Send a GET request, parsing the response body
     * @param path The path
     * @param sessionId Optional, provide the session ID. If this is left blank,
     * the session ID will be retrieved from local storage
     * @param ty The type of the response body
     */
    export async function getBody<T extends pb_1.Message>(path: string, sessionId: string | null, ty: typeof pb_1.Message): Promise<T | Response> {
        return await decodeBodyMaybe(await send(path, 'GET', null, sessionId), ty);
    }

    /**
     * Send a POST request, ignoring the response body
     * @param path The path
     * @param body Optional, the body to send
     * @param sessionId Optional, provide the session ID. If this is left blank,
     * the session ID will be retrieved from local storage
     */
    export async function post(path: string, body: pb_1.Message | null = null, sessionId: string | null): Promise<Response> {
        return await send(path, 'POST', body, sessionId);
    }

    /**
     * Send a POST request, parsing the response body
     * @param path The path
     * @param body Optional, the body to send
     * @param sessionId Optional, provide the session ID. If this is left blank,
     * the session ID will be retrieved from local storage
     * @param ty The type of the response body
     */
    export async function postBody<T extends pb_1.Message>(path: string, body: pb_1.Message | null = null, sessionId: string | null, ty: typeof pb_1.Message): Promise<T | Response> {
        return await decodeBodyMaybe(await send(path, 'POST', body, sessionId), ty);
    }

    /**
     * Send a DELETE request, ignoring the response body
     * @param path The path
     * @param body Optional, the body to send
     * @param sessionId Optional, provide the session ID. If this is left blank,
     * the session ID will be retrieved from local storage
     */
    export async function del(path: string, body: pb_1.Message | null = null, sessionId: string | null): Promise<Response> {
        return await send(path, 'DELETE', body, sessionId);
    }

    /**
     * Send a DELETE request, parsing the response body
     * @param path The path
     * @param body Optional, the body to send
     * @param sessionId Optional, provide the session ID. If this is left blank,
     * the session ID will be retrieved from local storage
     * @param ty The type of the response body
     */
    export async function delBody<T extends pb_1.Message>(path: string, body: pb_1.Message | null = null, sessionId: string | null, ty: typeof pb_1.Message): Promise<T | Response> {
        return await decodeBodyMaybe(await send(path, 'DELETE', body, sessionId), ty);
    }

    /**
     * Send a PATCH request, ignoring the response body
     * @param path The path
     * @param body Optional, the body to send
     * @param sessionId Optional, provide the session ID. If this is left blank,
     * the session ID will be retrieved from local storage
     */
    export async function patch(path: string, body: pb_1.Message | null = null, sessionId: string | null): Promise<Response> {
        return await send(path, 'PATCH', body, sessionId);
    }

    /**
     * Send a PATCh request, parsing the response body
     * @param path The path
     * @param body Optional, the body to send
     * @param sessionId Optional, provide the session ID. If this is left blank,
     * the session ID will be retrieved from local storage
     * @param ty The type of the response body
     */
    export async function patchBody<T extends pb_1.Message>(path: string, body: pb_1.Message | null = null, sessionId: string | null, ty: typeof pb_1.Message): Promise<T | Response> {
        return await decodeBodyMaybe(await send(path, 'PATCH', body, sessionId), ty);
    }
}