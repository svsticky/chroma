import {User, UserScope} from "@/generated/entity/user";
import {ListUserResponse} from "@/generated/payload/v1/user/list";
import {Http} from "@/http";
import {GetUserResponse} from "@/generated/payload/v1/user/get";

export interface UserModel {
    id: number,
    name: string,
}

export interface UserScopeModel {
    scope: string,
    grantedBy: number,
    grantedAt: number,
}

function protoUserToUserModel(user: User): UserModel {
    return <UserModel> {
        id: user.id,
        name: user.name,
    };
}

function protoUserScopeToUserScopeModel(scope: UserScope): UserScopeModel {
    return <UserScopeModel> {
        scope: scope.name,
        grantedBy: scope.grantedBy,
        grantedAt: scope.grantedAt,
    };
}

export async function listUsers(): Promise<UserModel[] | undefined> {
    const result = await Http.getBody<ListUserResponse>('/api/v1/user/list', null, ListUserResponse);
    if(result instanceof Response) {
        if(result.ok) {
            return [];
        } else {
            return undefined;
        }
    }

    return result.users.map(protoUserToUserModel);
}

export async function getUserScopes(userId: number): Promise<UserScopeModel[] | undefined> {
    const result = await Http.getBody<GetUserResponse>(`/api/v1/user?id=${userId}`, null, GetUserResponse);
    if(result instanceof Response) {
        if(result.ok) {
            return [];
        } else {
            return undefined;
        }
    }

    return result.scopes.map(protoUserScopeToUserScopeModel);
}