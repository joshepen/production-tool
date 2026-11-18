export interface User {
  id: number
  first_name: string
  last_name: string
  department_id: number
  department_name: string
  user_type_id: number
  hired_at: Date
}

export interface UserType {
  id: number
  name: string
}

export interface Department {
  id: number
  name: string
}

export interface Product {
  id: number
  name: string
}

export interface PrettyProduct {
  id: number
  name: string
  work_centers: string
}

export interface ProductOrder {
  id: number
  address: string
  product_id: number
  status_id: number
  created_at: Date
}

export interface WorkCenter {
  id: number
  name: string
  description?: string
}

export interface ProductWorkCenter {
  product_id: number
  step_order: number
  work_center_id: number
  work_center_name: string
}


export interface WorkOrder {
  id: number
  product_order_id: number
  work_center_id: number
  step_order: number
  status_id: number
  started_at?: Date
  completed_at?: Date
}

export interface PrettyWorkOrder {
  id: number
  product_order_id: number
  product_order_address: string
  work_center_id: number
  work_center_name: string
  step_order: number
  status_id: number
  status_name: string
  assignee_id?: number
  assignee_name?: string
  started_at?: Date
  completed_at?: Date
  estimated_hours?: number
}
