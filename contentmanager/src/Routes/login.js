import Button from 'react-bootstrap/Button'
import Form from 'react-bootstrap/Form';
import Container from 'react-bootstrap/Container';
import Row from 'react-bootstrap/Row';
import Col from 'react-bootstrap/Col';
import { useState } from 'react';

function Login(){

    const [emailString, setEmailString] = useState('')
    const [passwordString, setPasswordString] = useState('')
    const [alwaysConnected, SetAlwaysConnected] = useState('off')
    const [formValidated, SetformValidated] = useState(false) 

    function handleSubmit(event){
        
        const form = event.currentTarget;
        if (form.checkValidity() === false) {
          event.preventDefault();
          event.stopPropagation();
        }
    
        SetformValidated(true);
    }

return(
        
<Container className='d-flex align-items-center justify-content-center h-100'>

    <Col sm={6} lg={3}>
    
        <Row className="justify-content-center align-items-center">
            <h2 className='text-dark text-center'>Rustico CM</h2>
        </Row>
        <Form noValidate validated={formValidated} onSubmit={handleSubmit}>
            <Row className="justify-content-center mt-2">
                <Form.Group className='mb3' controlId='formLoginEmail'>
                    <Form.Label>Endereço de Email</Form.Label>
                    <Form.Control onChange={e => setEmailString(e.target.value)} type='email' required placeholder='digite@seu.email'/>
                    <Form.Control.Feedback type="invalid">
                        Digite um email válido.
                    </Form.Control.Feedback>
                </Form.Group>
            </Row>
            
            <Row className="justify-content-center mt-2">
                <Form.Group className='mb3' controlId='formLoginPassword'>
                    <Form.Label>Senha</Form.Label>
                    <Form.Control type='password' placeholder='**********' required onChange={e => setPasswordString(e.target.value)} />
                    <Form.Control.Feedback type="invalid">
                        Digite sua senha.
                    </Form.Control.Feedback>
                    <Form.Text className='text-muted'>Não compartilhe sua senha com ninguém!</Form.Text>
                </Form.Group>
            </Row>
            
            <Row className='mt-4'>
                <Form.Group className='mb3' controlId='formLoginCheck'>
                    <Form.Check onChange={e => {SetAlwaysConnected(e.target.value);console.log(alwaysConnected)}} type='checkbox' label='Manter Conectado' />
                </Form.Group>
            </Row>
            
            <Row className="justify-content-center mt-2" >
                <Button type='submit' className='btn-dark' size='lg'>Entrar</Button>
            </Row>
        </Form>
    </Col>
</Container>

);

}

export default Login;