import Gig from '../models/Gig.js';
import { generateId } from '../utils/idGenerator.js';
import { createMerkleTree, deserializeProof, getProof, serializeProof, verifyMerkleProof } from '../utils/merkleTreeUtils.js';
import { readOnlyProvider } from '../constants/providers.js';
import { getGigContract } from '../constants/contracts.js';
export const stageGig = async (req, res, next) => {
    try {
        const { clientAddress, title, skillCategory, preferredLocation, experienceLevel, projectDescription, contextLink, files, additionalProjectInfo, projectDuration, price, } = req.body;
        // Generate unique gig ID
        const databaseId = generateId(clientAddress, title, projectDescription);
        // Create staged gig object explicitly typed as IGig
        const stagedGig = {
            id: databaseId,
            clientAddress,
            title,
            skillCategory,
            preferredLocation,
            experienceLevel,
            projectDescription,
            contextLink,
            files,
            additionalProjectInfo,
            projectDuration,
            price,
            status: 'CREATED',
            createdAt: new Date(),
            merkleProof: [],
            merkleRoot: undefined,
        };
        const allGigs = await Gig.find().lean();
        const gigsForMerkle = [...allGigs, stagedGig];
        // Create Merkle Tree
        const { tree, root } = createMerkleTree(gigsForMerkle, 'gig');
        // Generate Merkle proof
        const proof = getProof(stagedGig, tree, 'gig');
        const serializedProof = serializeProof(proof);
        res.status(200).json({
            databaseId,
            merkleProof: serializedProof.map((p) => JSON.stringify(p)),
            merkleRoot: root,
        });
    }
    catch (error) {
        next(error);
    }
};
export const confirmGig = async (req, res, next) => {
    try {
        const { clientAddress, title, skillCategory, preferredLocation, experienceLevel, projectDescription, contextLink, files, additionalProjectInfo, projectDuration, price, databaseId, merkleRoot, merkleProof, } = req.body;
        // A readWriteProvider TO BE USED IN PLACE OF readOnlyProvider
        // const { walletProvider } = useAppKitProvider<Provider>("eip155");
        // const provider = getProvider(walletProvider);
        // const contract = getGigContract(provider);
        // Verify blockchain transaction
        const contract = getGigContract(readOnlyProvider);
        const gigInfo = await contract.getGigInfo(databaseId);
        if (gigInfo.client.toLowerCase() !== clientAddress.toLowerCase()) {
            res.status(400).json({ message: 'Gig not found on blockchain or client mismatch' });
            return;
        }
        // Verify Merkle proof against provided merkleRoot
        const gigData = {
            id: databaseId,
            clientAddress,
            title,
            skillCategory,
            preferredLocation,
            experienceLevel,
            projectDescription,
            contextLink,
            files,
            additionalProjectInfo,
            projectDuration,
            price,
            status: 'CREATED',
            createdAt: new Date(),
            merkleProof,
            merkleRoot,
        };
        const isValid = verifyMerkleProof(gigData, deserializeProof(merkleProof), merkleRoot, 'gig');
        if (!isValid) {
            res.status(400).json({ message: 'Invalid Merkle proof provided' });
            return;
        }
        // Create and save gig with provided merkleRoot and merkleProof
        const gig = new Gig({
            id: databaseId,
            clientAddress,
            title,
            skillCategory,
            preferredLocation,
            experienceLevel,
            projectDescription,
            contextLink,
            files,
            additionalProjectInfo,
            projectDuration,
            price,
            status: 'CREATED',
            createdAt: new Date(),
            merkleRoot,
            merkleProof,
        });
        // Save gig to database
        await gig.save();
        res.status(201).json({
            databaseId,
            merkleRoot,
            message: 'Gig successfully created',
        });
    }
    catch (error) {
        next(error);
    }
};
export const createGig = async (req, res, next) => {
    try {
        const { clientAddress, title, skillCategory, preferredLocation, experienceLevel, projectDescription, contextLink, files, additionalProjectInfo, projectDuration, price } = req.body;
        // Generate unique gig ID
        const databaseId = generateId(clientAddress, title, projectDescription);
        // Create gig object
        const gig = new Gig({
            id: databaseId,
            clientAddress,
            title,
            skillCategory,
            preferredLocation,
            experienceLevel,
            projectDescription,
            contextLink,
            files,
            additionalProjectInfo,
            projectDuration,
            price
        });
        // Save gig to database
        await gig.save();
        // Create Merkle Tree for all gigs with type specification
        const allGigs = await Gig.find().lean();
        const { tree, root } = createMerkleTree(allGigs, 'gig');
        // Generate Merkle proof with type
        const proof = getProof(gig.toObject(), tree, 'gig');
        // Serialize proof for storage
        const serializedProof = serializeProof(proof);
        gig.merkleProof = serializedProof.map(p => JSON.stringify(p));
        gig.merkleRoot = root;
        await gig.save();
        res.status(201).json({
            databaseId,
            merkleProof: gig.merkleProof,
            merkleRoot: root
        });
    }
    catch (error) {
        next(error);
    }
};
export const getGig = async (req, res, next) => {
    try {
        const { merkleRoot } = req.query; // Get root from blockchain
        const gig = await Gig.findOne({ id: req.params.databaseId });
        if (!gig) {
            res.status(404).json({ message: 'Gig not found' });
            return;
        }
        // Deserialize and verify the merkle proof
        const proof = deserializeProof(gig.merkleProof || []);
        const isValid = verifyMerkleProof(gig.toObject(), proof, gig.merkleRoot ?? '', 'gig');
        if (!isValid) {
            res.status(400).json({
                message: 'Invalid merkle proof. Data may have been tampered with.',
                gig
            });
            return;
        }
        res.json({
            verified: true,
            gig
        });
    }
    catch (error) {
        next(error);
    }
};
// export const getAllGigs = async (req: Request, res: Response, next: NextFunction): Promise<void> => {
//   try {
//     const gigs = await Gig.find();
//     res.json(gigs);
//   } catch (error) {
//     next(error);
//   }
// };
export const getAllGigs = async (req, res, next) => {
    try {
        const { page = 1, limit = 10, status, experienceLevel, skillCategory } = req.query;
        // Build query
        const query = {};
        if (status)
            query.status = status;
        if (experienceLevel)
            query.experienceLevel = experienceLevel;
        if (skillCategory)
            query.skillCategory = { $in: [skillCategory] };
        // Convert page and limit to numbers
        const pageNum = parseInt(page);
        const limitNum = parseInt(limit);
        const skip = (pageNum - 1) * limitNum;
        const gigs = await Gig.find(query)
            .sort({ createdAt: -1 }) // Most recent first
            .skip(skip)
            .limit(limitNum);
        // Get total count for pagination
        const total = await Gig.countDocuments(query);
        res.json({
            gigs,
            currentPage: pageNum,
            totalPages: Math.ceil(total / limitNum),
            totalGigs: total
        });
    }
    catch (error) {
        next(error);
    }
};
//# sourceMappingURL=gigController.js.map