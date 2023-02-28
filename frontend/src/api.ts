import {Http} from "@/http";
import {AccessResponse} from "@/generated/payload/v1/access";

const sessionIdKey = 'sessionid';
const isAdminKey = 'isadmin';
const beforeAuthUrlKey = 'beforeauth';

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

export async function checkLoggedIn(sessionId: string | null = null): Promise<LoginCheckResult | KoalaLoginUrl | null> {
    let r = await Http.getBody<AccessResponse>("/api/v1/access", sessionId, AccessResponse);

    if(r instanceof AccessResponse) {
        return new LoginCheckResult(r.admin);
    } else {
        switch(r.status) {
            case 401:
                let loginUrl = r.headers.get('location');
                return new KoalaLoginUrl(loginUrl!);
            default:
                console.error(`Invalid session ID`);
                return null;
        }
    }
}