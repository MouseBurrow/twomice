import { ApiError } from "./apiError.ts";
import type { AccountData, CommentData, PostData, ReplyData, TopicData } from "./types.ts";

export const API_BASE = "/api";

async function request<T>(
    path: string,
    options: RequestInit = {}
): Promise<T> {
    const headers = new Headers(options.headers);

    if (
        options.body &&
        !headers.has("Content-Type") &&
        !(options.body instanceof FormData)
    ) {
        headers.set("Content-Type", "application/json");
    }

    const res = await fetch(`${API_BASE}${path}?_=${Date.now()}`, {
        credentials: "include",
        headers,
        ...options,
    });

    const contentType = res.headers.get("content-type");
    if (!res.ok) {
        let payload: any = undefined;
        let message = res.statusText;
        let code: string | undefined;

        if (contentType?.includes("application/json")) {
            payload = await res.json();
            message = payload?.message ?? message;
            code = payload?.error ?? payload?.code;
        } else {
            message = await res.text();
        }

        throw new ApiError(path, res.status, message, code, payload);
    }

    if (contentType?.includes("application/json")) {
        return res.json();
    }

    return undefined as T;
}

export const api = {
    login: (body: { username: string; password: string }) =>
        request<void>("/login", {
            method: "POST",
            body: JSON.stringify(body),
        }),

    signup: (body: { username: string; password: string }) =>
        request<void>("/signup", {
            method: "POST",
            body: JSON.stringify(body),
        }),

    logout: () =>
        request<void>("/logout", {
            method: "POST",
        }),

    account: () =>
        request<AccountData>("/account", {
            method: "GET",
        }),

    createTopic: (body: { name: string, description: string }) =>
        request<void>("/mcf", {
            method: "POST",
            body: JSON.stringify(body),
        }),

    getTopic: (topic: string) =>
        request<TopicData>(`/mcf/${topic}`, {
            method: "GET",
        }),

    createPost: (topic: string, body: { title: string, content: string }) =>
        request<void>(`/mcf/${topic}/nib`, {
            method: "POST",
            body: JSON.stringify(body),
        }),

    getPost: (topic: string, postId: string) =>
        request<PostData>(`/mcf/${topic}/nib/${postId}`),

    createComment: (topic: string, post: string, body: { content: string }) =>
        request<void>(`/mcf/${topic}/nib/${post}/sqk`, {
            method: "POST",
            body: JSON.stringify(body),
        }),

    createReply: (
        topic: string,
        post: string,
        comment: string,
        body: { content: string }
    ) =>
        request<void>(`/mcf/${topic}/nib/${post}/sqk/${comment}/echoes`, {
            method: "POST",
            body: JSON.stringify(body),
        }),

    getReplies: (topic: string, post: string, comment: string) =>
        request<ReplyData[]>(`/mcf/${topic}/nib/${post}/sqk/${comment}/echoes`),

    getAllTopics: () =>
        request<TopicData[]>("/mcf", {
            method: "GET",
        }),

    getAllPosts: (topic: string) =>
        request<PostData[]>(`/mcf/${topic}/nib`, {
            method: "GET",
        }),

    getAllComments: (topic: string, post: string) =>
        request<CommentData[]>(
            `/mcf/${topic}/nib/${post}/sqk`, {
                method: "GET",
            }
        ),
};