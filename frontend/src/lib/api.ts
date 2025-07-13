const API_BASE_URL = 'http://localhost:3001/api';

export interface Student {
  id: string;
  name: string;
  age: number;
  grade: number;
}

export interface Statistics {
  total_students: number;
  average_grade: number;
  highest_grade: number;
  lowest_grade: number;
}

export interface CreateStudentRequest {
  name: string;
  age: number;
  grade: number;
}

class ApiService {
  private async request<T>(endpoint: string, options?: RequestInit): Promise<T> {
    const response = await fetch(`${API_BASE_URL}${endpoint}`, {
      headers: {
        'Content-Type': 'application/json',
        ...options?.headers,
      },
      ...options,
    });

    if (!response.ok) {
      throw new Error(`HTTP error! status: ${response.status}`);
    }

    return response.json();
  }

  // Get all students
  async getStudents(): Promise<Student[]> {
    return this.request<Student[]>('/students');
  }

  // Get a single student
  async getStudent(id: string): Promise<Student> {
    return this.request<Student>(`/students/${id}`);
  }

  // Create a new student
  async createStudent(student: CreateStudentRequest): Promise<Student> {
    return this.request<Student>('/students', {
      method: 'POST',
      body: JSON.stringify(student),
    });
  }

  // Update a student
  async updateStudent(id: string, student: CreateStudentRequest): Promise<Student> {
    return this.request<Student>(`/students/${id}`, {
      method: 'PUT',
      body: JSON.stringify({ ...student, id }),
    });
  }

  // Delete a student
  async deleteStudent(id: string): Promise<{ message: string }> {
    return this.request<{ message: string }>(`/students/${id}`, {
      method: 'DELETE',
    });
  }

  // Get statistics
  async getStatistics(): Promise<Statistics> {
    return this.request<Statistics>('/statistics');
  }
}

export const apiService = new ApiService(); 