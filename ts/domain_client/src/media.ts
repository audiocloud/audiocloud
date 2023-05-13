import memoizeOne from "memoize-one";
import { z } from "zod";
export const MediaDownloadSpec = memoizeOne(() => z.object({fromUrl: z.string(), sha256: z.string(), size: z.number().int(), }));
export type MediaDownloadSpec = z.infer<ReturnType<typeof MediaDownloadSpec>>;

export const MediaDownloadState = memoizeOne(() => z.object({done: z.union([z.lazy(MediaSpec), z.null(), ]), error: z.union([z.string(), z.null(), ]), progress: z.number(), updatedAt: z.coerce.date(), }));
export type MediaDownloadState = z.infer<ReturnType<typeof MediaDownloadState>>;

export const MediaId = memoizeOne(() => z.string());
export type MediaId = z.infer<ReturnType<typeof MediaId>>;

export const MediaSpec = memoizeOne(() => z.object({id: z.lazy(MediaId), sha256: z.string(), }));
export type MediaSpec = z.infer<ReturnType<typeof MediaSpec>>;

export const MediaUploadSpec = memoizeOne(() => z.object({toUrl: z.string(), }));
export type MediaUploadSpec = z.infer<ReturnType<typeof MediaUploadSpec>>;

export const MediaUploadState = memoizeOne(() => z.object({error: z.union([z.string(), z.null(), ]), progress: z.number(), updatedAt: z.coerce.date(), uploaded: z.boolean(), }));
export type MediaUploadState = z.infer<ReturnType<typeof MediaUploadState>>;
