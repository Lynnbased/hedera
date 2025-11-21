import mongoose from 'mongoose';
let isConnected = false;
const connectDB = async () => {
    if (isConnected) {
        console.log('Using existing MongoDB connection');
        return;
    }
    try {
        if (!process.env.MONGODB_URI) {
            console.error('MONGODB_URI is not defined');
            throw new Error('MONGODB_URI is not defined');
        }
        console.log('Connecting to MongoDB with URI:', process.env.MONGODB_URI.replace(/:.*@/, ':<hidden>@')); // Hide password
        await mongoose.connect(process.env.MONGODB_URI, {
            serverSelectionTimeoutMS: 8000, // Fit Vercel Hobby plan
            maxPoolSize: 10,
            retryWrites: true,
            w: 'majority',
            appName: 'CraftLink',
        });
        isConnected = true;
        console.log('MongoDB connected successfully');
    }
    catch (error) {
        console.error('MongoDB connection error:', error);
        isConnected = false; // Allow retries
        throw error;
    }
};
export default connectDB;
//# sourceMappingURL=db.js.map