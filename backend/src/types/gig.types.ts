import { Request } from "express";

export interface IGig {
  id: string;
  clientAddress: string;
  title: string;
  skillCategory: string[];
  preferredLocation: string;
  experienceLevel: 'BEGINNER' | 'INTERMEDIATE' | 'EXPERT';
  projectDescription: string;
  contextLink?: string;
  additionalProjectInfo?: string;
  files: {
    type: 'IMAGE' | 'VIDEO' | 'PDF';
    url: string;
  }[];
  projectDuration: {
    weeks: number;
  };
  price: number;
  status: 'CREATED' | 'IN_PROGRESS' | 'COMPLETED' | 'DISPUTED';
  createdAt: Date;
  merkleProof?: string[];
  merkleRoot?: string;
}

export interface GigCreationRequest extends Request {
  body: Omit<IGig,
    'id' |
    'status' |
    'createdAt' |
    'merkleProof' |
    'merkleRoot'
  >;
}

export interface ConfirmGigRequest extends IGig {
  databaseId: string;
  merkleRoot: string;
  merkleProof: string[];
}