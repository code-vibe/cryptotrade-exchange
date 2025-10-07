import React, { useState } from 'react';
import { Link, useNavigate } from 'react-router-dom';
import { useForm } from 'react-hook-form';
import { zodResolver } from '@hookform/resolvers/zod';
import { z } from 'zod';
import { EyeIcon, EyeSlashIcon } from '@heroicons/react/24/outline';
import { useMutation } from '@tanstack/react-query';
import toast from 'react-hot-toast';
import { apiClient } from '../../services/api';
import { useAuthStore } from '../../stores';
import type { LoginRequest } from '../../types';

const loginSchema = z.object({
  email: z.string().email('Invalid email address'),
  password: z.string().min(8, 'Password must be at least 8 characters'),
  totpCode: z.string().optional(),
});

type LoginFormData = z.infer<typeof loginSchema>;

const Login: React.FC = () => {
  const [showPassword, setShowPassword] = useState(false);
  const [show2FA, setShow2FA] = useState(false);
  const navigate = useNavigate();
  const { login } = useAuthStore();

  const {
    register,
    handleSubmit,
    formState: { errors },
  } = useForm<LoginFormData>({
    resolver: zodResolver(loginSchema),
  });

  const loginMutation = useMutation({
    mutationFn: (data: LoginRequest) => apiClient.login(data),
    onSuccess: (response) => {
      login(response.user, response.accessToken, response.refreshToken);
      toast.success('Welcome back! Login successful');
      navigate('/', { replace: true });
    },
    onError: (error: any) => {
      if (error.response?.data?.error === 'TWO_FACTOR_REQUIRED') {
        setShow2FA(true);
        toast.error('Please enter your 2FA code');
      } else {
        toast.error(error.response?.data?.message || 'Login failed');
      }
    },
  });

  const onSubmit = (data: LoginFormData) => {
    loginMutation.mutate(data);
  };

  return (
    <>
      <div className="text-center" style={{ marginBottom: '32px' }}>
        <h2 style={{ fontSize: '1.875rem', fontWeight: '700', color: '#111827', marginBottom: '8px' }}>
          Welcome Back
        </h2>
        <p style={{ color: '#6b7280', fontWeight: '500' }}>
          Sign in to your trading account
        </p>
      </div>

      <form onSubmit={handleSubmit(onSubmit)} style={{ width: '100%' }}>
        {/* Email Field */}
        <div className="form-group">
          <label htmlFor="email" className="form-label">
            Email Address
          </label>
          <input
            id="email"
            type="email"
            autoComplete="email"
            placeholder="Enter your email address"
            className="input-field"
            {...register('email')}
          />
          {errors.email && (
            <p className="form-error">{errors.email.message}</p>
          )}
        </div>

        {/* Password Field */}
        <div className="form-group">
          <label htmlFor="password" className="form-label">
            Password
          </label>
          <div style={{ position: 'relative' }}>
            <input
              id="password"
              type={showPassword ? 'text' : 'password'}
              autoComplete="current-password"
              placeholder="Enter your password"
              className="input-field"
              style={{ paddingRight: '48px' }}
              {...register('password')}
            />
            <button
              type="button"
              style={{
                position: 'absolute',
                right: '12px',
                top: '50%',
                transform: 'translateY(-50%)',
                background: 'none',
                border: 'none',
                cursor: 'pointer',
                color: '#9ca3af'
              }}
              onClick={() => setShowPassword(!showPassword)}
            >
              {showPassword ? (
                <EyeSlashIcon style={{ width: '20px', height: '20px' }} />
              ) : (
                <EyeIcon style={{ width: '20px', height: '20px' }} />
              )}
            </button>
          </div>
          {errors.password && (
            <p className="form-error">{errors.password.message}</p>
          )}
        </div>

        {/* 2FA Field */}
        {show2FA && (
          <div className="form-group">
            <label htmlFor="totpCode" className="form-label">
              Two-Factor Authentication Code
            </label>
            <input
              id="totpCode"
              type="text"
              placeholder="Enter your 2FA code"
              className="input-field"
              style={{ textAlign: 'center', letterSpacing: '0.1em', fontFamily: 'monospace', fontSize: '18px' }}
              maxLength={6}
              {...register('totpCode')}
            />
            <p style={{ fontSize: '12px', color: '#6b7280', marginTop: '8px', textAlign: 'center' }}>
              Enter the 6-digit code from your authenticator app
            </p>
            {errors.totpCode && (
              <p className="form-error" style={{ textAlign: 'center' }}>{errors.totpCode.message}</p>
            )}
          </div>
        )}

        {/* Remember Me & Forgot Password */}
        <div style={{ display: 'flex', alignItems: 'center', justifyContent: 'space-between', marginBottom: '24px' }}>
          <label style={{ display: 'flex', alignItems: 'center', cursor: 'pointer' }}>
            <input
              type="checkbox"
              style={{ width: '16px', height: '16px', marginRight: '8px', accentColor: '#3b82f6' }}
            />
            <span style={{ fontSize: '14px', color: '#374151', fontWeight: '500' }}>Remember me</span>
          </label>
          <Link
            to="/forgot-password"
            style={{
              fontSize: '14px',
              color: '#3b82f6',
              fontWeight: '600',
              textDecoration: 'none',
              transition: 'color 0.2s ease'
            }}
          >
            Forgot password?
          </Link>
        </div>

        {/* Submit Button */}
        <button
          type="submit"
          disabled={loginMutation.isPending}
          className="btn-primary"
          style={{ width: '100%', marginBottom: '24px' }}
        >
          {loginMutation.isPending ? (
            <div style={{ display: 'flex', alignItems: 'center', justifyContent: 'center' }}>
              <div className="loading-spinner" style={{ marginRight: '8px' }}></div>
              Signing in...
            </div>
          ) : (
            'Sign In'
          )}
        </button>
      </form>

      {/* Sign Up Link */}
      <div className="text-center">
        <p style={{ color: '#6b7280' }}>
          New to CryptoTrade Exchange?{' '}
          <Link
            to="/register"
            style={{
              color: '#3b82f6',
              fontWeight: '600',
              textDecoration: 'none',
              transition: 'color 0.2s ease'
            }}
          >
            Create an account
          </Link>
        </p>
      </div>
    </>
  );
};

export default Login;
