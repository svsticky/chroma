import {Http} from "@/http";
import {AccessResponse} from "@/generated/payload/v1/access";

const sessionIdKey = 'sessionid';
const isAdminKey = 'isadmin';
const beforeAuthUrlKey = 'beforeauth';
const isDarkModeKey = 'darkmode';

export const errorText = "Something went wrong";

export namespace Storage {
    export function getSessionId(): string | null {
        return window.localStorage.getItem(sessionIdKey);
    }

    export function setSessionId(id: string) {
        window.localStorage.setItem(sessionIdKey, id);
    }

    export function isAdmin(): boolean {
        return window.localStorage.getItem(isAdminKey) === 'true'
    }

    export function setAdmin(admin: boolean) {
        if(admin) {
            window.localStorage.setItem(isAdminKey, 'true');
        } else {
            window.localStorage.removeItem(isAdminKey);
        }
    }

    export function getBeforeAuthUrl(): string | null {
        return window.localStorage.getItem(beforeAuthUrlKey);
    }

    export function setBeforeAuthUrl(url: string) {
        window.localStorage.setItem(beforeAuthUrlKey, url);
    }

    export function getIsDarkMode(): boolean {
        return window.localStorage.getItem(isDarkModeKey) != null;
    }

    export function setIsDarkMode(darkMode: boolean) {
        if(darkMode) {
            window.localStorage.setItem(isDarkModeKey, "true");
        } else {
            window.localStorage.removeItem(isDarkModeKey);
        }
    }
}

export class LoginCheckResult {
    isAdmin: boolean;

    constructor(isAdmin: boolean) {
        this.isAdmin = isAdmin;
    }
}

export class KoalaLoginUrl {
    url: string;

    constructor(url: string) {
        this.url = url;
    }
}

export async function checkScope(scope: string): Promise<boolean | undefined> {
    const result = await Http.getBody<AccessResponse>(`/api/v1/access?scope=${scope}`, null, AccessResponse);
    if(result instanceof Response) {
        if(result.status == 429) {
            console.error("We got a 429 from the server, waiting for a bit and trying again.")
            await new Promise( resolve => setTimeout(resolve, 2000));
            return await checkScope(scope);
        } else {
            return undefined;
        }
    }

    return result.hasRequestedScope;
}

export async function checkLoggedIn(sessionId: string | null = null): Promise<LoginCheckResult | KoalaLoginUrl | null> {
    let r = await Http.getBody<AccessResponse>("/api/v1/access", sessionId, AccessResponse);

    if(r instanceof AccessResponse) {
        return new LoginCheckResult(r.admin);
    } else {
        switch(r.status) {
            case 200:
                // Need to perform the status 200 check, as protobuf will return an empty
                // response if the value isAdmin is falses
                return new LoginCheckResult(false);
            case 401:
                let loginUrl = r.headers.get('location');
                return new KoalaLoginUrl(loginUrl!);
            case 429:
                // wait a bit and try again
                console.error("We got a 429 from the server, waiting for a bit and trying again.")
                await new Promise( resolve => setTimeout(resolve, 2000));
                return await checkLoggedIn(sessionId);
            default:
                console.error(`Invalid session ID`);
                return null;
        }
    }
}