import { ethers } from "ethers";
import CustomDexABI from "../utils/CustomDex.json";
import CustomTokenABI from "../utils/CustomToken.json";

export const TokenContract = async(address) =>{
    const provider = new ethers.providers.Web3Provider(window.ethereum);
    const { ethereum }= window;

    if(ethereum){
        const signer = provider.getSigner();
        const contractReader = new ethers.Contract(address, CustomTokenABI.abi, signer);
        return contractReader;
    }

    
};

export const contract = async () =>{
    const provider = new ethers.providers.Web3Provider(window.ethereum);
    const { ethereum }= window;

    if(ethereum){
        const signer = provider.getSigner();
        const contractReader = new ethers.Contract(
          "0xdFBD5Df2200490743c55707CFf1Ff28a63C87839",
          CustomDexABI.abi,
          signer
        );
        
        return contractReader;
    }
};


