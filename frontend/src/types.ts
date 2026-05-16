export type AuthState =
    { status: "unknown" } |
    { status: "guest" } |
    { status: "user"; info: AccountData } |
    { status: "admin"; info: AccountData };

export type AccountData = {
    username: string;
    is_admin: boolean;
    created_at: string;
    updated_at: string;
}

export type TopicData = {
    name: string;
    description: string;
    created_at: string;
    deleted: boolean;
};

export type PostData = {
    title: string;
    slug: string;
    content: string;
    image_url: string;
    created_at: string;
    deleted: boolean;
};

export type CommentData = {
    hash: string;
    content: string;
    created_at: string;
    deleted: boolean;
}

export type ReplyData = {
    hash: string;
    content: string;
    created_at: string;
    deleted: boolean;
};