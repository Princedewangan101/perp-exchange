export function isValidEmail(email: string): boolean {
    // Regex for standard RFC 5322 email validation
    const emailRegex = /^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$/;
    
    if (!email || email.trim().length === 0) {
        return false;
    }
    
    return emailRegex.test(email.trim());
}