export interface User {
  id: number
  first_name: string
  last_name: string
  department_id: number
  department_name: string
  hired_at: Date
}

export interface Department {
  id: number
  name: string
}

export interface Product {
  id: number
  name: string
}

export interface ProductOrder {
  id: number
  address: string
  product_id: number
  status_id: number
  created_at: Date
}
