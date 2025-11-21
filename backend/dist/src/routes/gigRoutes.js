import express from 'express';
import { createGig, stageGig, confirmGig, getAllGigs, getGig } from '../controllers/gigController.js';
import { validateGigCreation } from '../middlewares/validationMiddleware.js';
const router = express.Router();
router.post('/gigs/stage', validateGigCreation, stageGig);
router.post('/gigs/confirm', validateGigCreation, confirmGig);
router.get('/gigs/:databaseId', getGig);
router.get('/gigs', getAllGigs);
router.post('/gigs', validateGigCreation, createGig); // Deprecated endpoint
// Get all gigs
// fetch('/api/gigs')
// // With pagination
// fetch('/api/gigs?page=1&limit=10')
// // With filters
// fetch('/api/gigs?status=CREATED&experienceLevel=EXPERT&skillCategory=Web Development')
export default router;
//# sourceMappingURL=gigRoutes.js.map