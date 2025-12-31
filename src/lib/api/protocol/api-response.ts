export interface ApiResponse<T> {
  success: boolean;
  results: T;
  stats: string;
  timestamp: string;
}
